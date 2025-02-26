use kids_bank_lib::{AsyncAccountHandler, DynamoClient};
use kids_bank_sam::response_error;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;

async fn get_acct(request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name =
        env::var("TABLE_NAME").map_err(|_| response_error(500, "TABLE_NAME must be set"))?;
    let dc = DynamoClient::new(&config, &table_name)
        .map_err(|_| response_error(500, "Failed to create DynamoClient"))?;
    let query_parameters = request.query_string_parameters();
    let email = query_parameters
        .first("email")
        .ok_or(response_error(400, "Missing 'email' query parameter"))?;
    let acct = dc
        .get_account_by_email_async(email)
        .await
        .map_err(|e| response_error(500, &format!("Failed to create account {}", e)))?;
    Ok(Response::builder()
        .status(200)
        .body(serde_json::to_string(&acct)?.into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(get_acct)).await
}
