use kids_bank_lib::{dynamo_client::DynamoClient, AsyncAccountHandler};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use std::env;

async fn get_accts(_request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dc_res = DynamoClient::new(&config, &table_name);
    if dc_res.is_err() {
        return Err("Failed to create DynamoClient".into());
    }
    let dc = dc_res.unwrap();
    let accts_res = dc.get_accounts_async().await;
    match accts_res {
        Ok(a) => Ok(Response::builder()
            .status(200)
            .body(serde_json::to_string(&a)?.into())?),
        Err(e) => Err(format!("Failed to get accounts {}", e).into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(get_accts)).await
}
