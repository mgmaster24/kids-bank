use kids_bank_lib::{dynamo_client::DynamoClient, AsyncAccountHandler};
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the  following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serve:rless-rust-demo/
async fn deposit(request: Request) -> Result<Response<Body>, Error> {
    // Prepare the response
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    if let Ok(dc) = DynamoClient::new(&config, &table_name) {
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
            Ok(a) => return Ok(Response::builder().status(200).body(a.to_string().into())?),
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

    run(service_fn(|request: Request| deposit(request))).await
}
