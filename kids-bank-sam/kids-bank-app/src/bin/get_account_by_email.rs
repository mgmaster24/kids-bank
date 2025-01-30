use kids_bank_lib::{dynamo_client::DynamoClient, AsyncAccountHandler};
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;

async fn get_acct(request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dc_res = DynamoClient::new(&config, &table_name);
    if dc_res.is_err() {
        return Err("Failed to create DynamoClient".into());
    }
    let dc = dc_res.unwrap();
    let query_parameters = request.query_string_parameters();
    let email = query_parameters.first("email");
    let email = match email {
        Some(e) => e,
        None => {
            return Ok(Response::builder()
                .status(400)
                .body("expected email parameter to be present".into())?)
        }
    };
    let acct_res = dc.get_account_by_email_async(email).await;
    match acct_res {
        Ok(a) => Ok(Response::builder()
            .status(200)
            .body(serde_json::to_string(&a)?.into())?),
        Err(e) => Err(format!("Failed to create account {}", e).into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(get_acct)).await
}
