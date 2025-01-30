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
    let id = query_parameters.first("id");
    if let Some(i) = id {
        let acct_res = dc.get_account_by_id_async(i).await;
        match acct_res {
            Ok(a) => {
                return Ok(Response::builder()
                    .status(200)
                    .body(serde_json::to_string(&a)?.into())?);
            }
            Err(e) => return Err(format!("Failed to get account {}", e).into()),
        }
    }
    if let Some(email) = query_parameters.first("email") {
        let acct_res = dc.get_account_by_email_async(email).await;
        match acct_res {
            Ok(a) => {
                return Ok(Response::builder()
                    .status(200)
                    .body(serde_json::to_string(&a)?.into())?)
            }
            Err(e) => return Err(format!("Failed to get account {}", e).into()),
        }
    }

    Ok(Response::builder()
        .status(400)
        .body("no id or email query parameter were provided".into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(get_acct)).await
}
