use kids_bank_lib::{AsyncAccountHandler, DynamoClient};
use kids_bank_sam::response_error;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;

async fn deposit(request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name =
        env::var("TABLE_NAME").map_err(|_| response_error(500, "TABLE_NAME must be set"))?;
    let query_parameters = request.query_string_parameters();
    let id = query_parameters
        .first("id")
        .ok_or(response_error(400, "Missing 'id' query parameter"))?;
    let amount = query_parameters
        .first("amount")
        .ok_or(response_error(400, "Missing amount query parameter"))?
        .parse::<f64>()
        .map_err(|_| response_error(400, "Invalid 'amount', must be a valid f64"))?;
    let dc = DynamoClient::new(&config, &table_name)
        .map_err(|_| response_error(500, "Failed to create DynamoClient"))?;
    let amount = dc
        .deposit_async(id, amount)
        .await
        .map_err(|e| response_error(500, &format!("Failed to withdraw {}", e)))?;
    let json_resp = serde_json::json!({"balance": amount});
    let serialized = serde_json::to_string(&json_resp)?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(serialized.into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(deposit)).await
}
