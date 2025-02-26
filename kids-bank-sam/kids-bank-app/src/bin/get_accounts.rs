use kids_bank_lib::{AsyncAccountHandler, DynamoClient};
use kids_bank_sam::response_error;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use std::env;

async fn get_accts(_request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name =
        env::var("TABLE_NAME").map_err(|_| response_error(500, "TABLE_NAME must be set"))?;
    let dc = DynamoClient::new(&config, &table_name)
        .map_err(|_| response_error(500, "Failed to create DynamoClient"))?;
    let accts = dc
        .get_accounts_async()
        .await
        .map_err(|e| response_error(500, &format!("Failed to get accounts {}", e)))?;
    Ok(Response::builder()
        .status(200)
        .body(serde_json::to_string(&accts)?.into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(get_accts)).await
}
