use kids_bank_lib::{AsyncAccountHandler, DynamoClient};
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;

async fn deposit(request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dc_res = DynamoClient::new(&config, &table_name);
    if dc_res.is_err() {
        return Err("Failed to create DynamoClient".into());
    }
    let dc = dc_res.unwrap();
    let query_parameters = request.query_string_parameters();
    let id = query_parameters
        .first("id")
        .expect("id query parameter should exist");
    let amount = query_parameters
        .first("amount")
        .expect("amount query parameter should exist");
    let amount = amount.parse::<f64>().expect("amount should be f64");
    let acct_res = dc.deposit_async(id, amount).await;
    match acct_res {
        Ok(a) => Ok(Response::builder().status(200).body(a.to_string().into())?),
        Err(e) => Err(format!("Failed to create account {}", e).into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(deposit)).await
}
