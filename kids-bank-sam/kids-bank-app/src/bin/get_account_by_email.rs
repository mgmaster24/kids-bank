use kids_bank_lib::AsyncAccountHandler;
use kids_bank_sam::DynamoClient;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the  following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serve:rless-rust-demo/
async fn get_acct(request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    if let Ok(dc) = DynamoClient::new(&config, &table_name) {
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

    run(service_fn(|request: Request| get_acct(request))).await
}
