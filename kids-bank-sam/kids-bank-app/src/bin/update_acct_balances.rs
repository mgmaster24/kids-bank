use lambda_http::{run, service_fn, Body, Error, Request, Response};

async fn update_account_balances(_request: Request) -> Result<Response<Body>, Error> {
    println!("Updating balances");

    Ok(Response::builder()
        .status(500)
        .body("Not implemented".into())?)
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

    run(service_fn(|request: Request| {
        update_account_balances(request)
    }))
    .await
}
