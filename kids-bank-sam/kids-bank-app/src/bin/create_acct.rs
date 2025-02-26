use kids_bank_lib::DynamoClient;
use kids_bank_sam::response_error;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug, Default)]
struct CreateAccountRequestBody {
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    password: String,
}

async fn create_acct(request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name =
        env::var("TABLE_NAME").map_err(|_| response_error(500, "TABLE_NAME must be set"))?;
    let body_bytes = request.body().to_vec();
    let req_body: CreateAccountRequestBody = serde_json::from_slice(&body_bytes)
        .map_err(|e| response_error(400, &format!("Invalid request body: {}", e)))?;
    let dc = DynamoClient::new(&config, &table_name)
        .map_err(|_| response_error(500, "Failed to create DynamoClient"))?;
    dc.create_new_account(&req_body.name, &req_body.email, &req_body.password)
        .await
        .map_err(|e| response_error(500, &format!("Failed to create account {}", e)))?;
    Ok(Response::builder()
        .status(200)
        .body("account created".into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(create_acct)).await
}
