use kids_bank_lib::{dynamo_client::DynamoClient, AsyncAccountHandler};
use kids_bank_sam::CreateAccountRequestBody;
use lambda_http::{run, service_fn, Body, Error, Request, RequestPayloadExt, Response};
use std::env;

async fn create_acct(request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    if let Ok(dc) = DynamoClient::new(&config, &table_name) {
        match request.payload::<CreateAccountRequestBody>() {
            Ok(account_opt) => match account_opt {
                Some(account_req) => {
                    let acct_res = dc
                        .create_account_async(
                            &account_req.name,
                            &account_req.email,
                            &account_req.password,
                        )
                        .await;
                    match acct_res {
                        Ok(a) => {
                            return Ok(Response::builder()
                                .status(200)
                                .body(serde_json::to_string(&a)?.into())?)
                        }
                        Err(e) => {
                            let err_str = format!("Failed to create account {e:#}");
                            return Ok(Response::builder().status(500).body(err_str.into())?);
                        }
                    };
                }
                None => {
                    let err_str = "Failed to create account request";
                    return Ok(Response::builder().status(500).body(err_str.into())?);
                }
            },
            Err(e) => {
                let err_str = format!("Failed to deserialize payload: {e:#}");
                return Ok(Response::builder().status(500).body(err_str.into())?);
            }
        }
    }

    Ok(Response::builder()
        .status(500)
        .body("Failed to created the dynamodb client".into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(|request: Request| create_acct(request))).await
}
