use jsonwebtoken::{encode, EncodingKey, Header};
use kids_bank_lib::{dynamo_client::DynamoClient, AsyncAccountHandler};
use kids_bank_sam::{secrets_manager, Claims};
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
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let body = request.into_body();
    let dc_res = DynamoClient::new(&config, &table_name);
    if dc_res.is_err() {
        return Err("Failed to create DynamoClient".into());
    }
    let dc = dc_res.unwrap();

    match body {
        Body::Text(t) => {
            let d_body: Result<LoginRequestBody, serde_json::Error> = serde_json::from_str(&t);
            if let Err(e) = d_body {
                return Err(format!("Failed to desrialize the request body, {}", e).into());
            }

            let lr = d_body.unwrap();
            let acct_res = dc.get_account_by_email_async(&lr.email).await;
            if let Err(e) = acct_res {
                let err_str = format!("Failed to find account by email. {e:#}");
                return Ok(Response::builder().status(500).body(err_str.into())?);
            }

            let account = acct_res.unwrap();
            if !account.user.are_pws_equal(&lr.password) {
                return Ok(Response::builder()
                    .status(401)
                    .body("Incorrect password".into())?);
            }

            let token_secret_res = secrets_manager::get_token_secret().await;
            if let Err(e) = token_secret_res {
                return Ok(Response::builder().status(500).body(e.into())?);
            }

            let claims = Claims {
                sub: account.user.email().to_string(),
                exp: chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::hours(1))
                    .unwrap()
                    .timestamp() as usize,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(token_secret_res.unwrap().as_ref()),
            );

            if let Err(e) = token {
                return Ok(Response::builder()
                    .status(500)
                    .body(format!("Failed to generate token. error: {}", e).into())?);
            }

            let json_resp = serde_json::json!({"token": token.unwrap(), "account": account});
            let serialized = serde_json::to_string(&json_resp)?;
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(serialized.into())?)
        }
        _ => Ok(Response::builder()
            .status(500)
            .body("Unexpected body type".into())?),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(login)).await
}
