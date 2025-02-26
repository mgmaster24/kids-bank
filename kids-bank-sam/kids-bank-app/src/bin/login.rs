use jsonwebtoken::{encode, EncodingKey, Header};
use kids_bank_lib::{AsyncAccountHandler, DynamoClient};
use kids_bank_sam::{get_token_secret, response_error, Claims};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug, Default)]
struct LoginRequestBody {
    #[serde(default)]
    email: String,
    #[serde(default)]
    password: String,
}

async fn login(request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name =
        env::var("TABLE_NAME").map_err(|_| response_error(500, "TABLE_NAME must be set"))?;
    let body_bytes = request.body().to_vec();
    let lr: LoginRequestBody = serde_json::from_slice(&body_bytes)
        .map_err(|e| response_error(400, &format!("Invalid request body, {}", e)))?;
    let dc = DynamoClient::new(&config, &table_name)
        .map_err(|e| response_error(500, &format!("Failed to create DynamoClient, {}", e)))?;
    let account = dc
        .get_account_by_email_async(&lr.email)
        .await
        .map_err(|_| response_error(404, "Account not found"))?;
    if !account.user.are_pws_equal(&lr.password) {
        return Err(response_error(401, "Incorrect password"));
    }
    let token_secret = get_token_secret()
        .await
        .map_err(|_| response_error(500, "Failed to retrieve token secret"))?;
    let claims = Claims::new(
        account.user.email().to_string(),
        chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(1))
            .unwrap()
            .timestamp() as usize,
    );
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(token_secret.as_bytes()),
    )
    .map_err(|e| response_error(500, &format!("Failed to generate token: {}", e)))?;
    let json_resp = serde_json::json!({"token": token, "account": account});
    let serialized = serde_json::to_string(&json_resp)?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(serialized.into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(login)).await
}
