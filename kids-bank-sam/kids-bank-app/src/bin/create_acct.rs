use kids_bank_lib::{dynamo_client::DynamoClient, AsyncAccountHandler};
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
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    if let Ok(dc) = DynamoClient::new(&config, &table_name) {
        let body = request.into_body();
        match body {
            Body::Text(t) => {
                let d_body: Result<CreateAccountRequestBody, serde_json::Error> =
                    serde_json::from_str(&t);
                if let Ok(c) = d_body {
                    let acct_res = dc.create_new_account(&c.email, &c.name, &c.password).await;
                    match acct_res {
                        Ok(_) => {
                            return Ok(Response::builder()
                                .status(200)
                                .body("account created".into())?)
                        }
                        Err(e) => {
                            let err_str = format!("Failed to create account {e:#}");
                            return Ok(Response::builder().status(500).body(err_str.into())?);
                        }
                    };
                }

                return Ok(Response::builder()
                    .status(500)
                    .body("Failed to desrialize the request body".into())?);
            }
            _ => {
                return Ok(Response::builder()
                    .status(500)
                    .body("Failed to created the dynamodb client".into())?)
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
