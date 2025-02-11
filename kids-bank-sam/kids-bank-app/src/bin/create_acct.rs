use kids_bank_lib::DynamoClient;
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
    let dc_res = DynamoClient::new(&config, &table_name);
    if dc_res.is_err() {
        return Err("Failed to create DynamoClient".into());
    }

    let dc = dc_res.unwrap();
    match request.into_body() {
        Body::Text(t) => {
            let d_body: Result<CreateAccountRequestBody, serde_json::Error> =
                serde_json::from_str(&t);
            if let Err(e) = d_body {
                return Err(format!("Failed to desrialize the request body. error {}", e).into());
            }

            let c = d_body.unwrap();
            let acct_res = dc.create_new_account(&c.name, &c.email, &c.password).await;
            match acct_res {
                Ok(_) => Ok(Response::builder()
                    .status(200)
                    .body("account created".into())?),
                Err(e) => Err(format!("Failed to create account {}", e).into()),
            }
        }
        _ => Err("Failed to read body for request".into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(create_acct)).await
}
