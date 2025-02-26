use kids_bank_lib::{AsyncAccountHandler, DynamoClient};
use kids_bank_sam::response_error;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;

async fn get_acct(request: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let table_name =
        env::var("TABLE_NAME").map_err(|_| response_error(500, "TABLE_NAME must be set"))?;
    let dc = DynamoClient::new(&config, &table_name)
        .map_err(|_| response_error(500, "Failed to create DynamoClient"))?;
    let query_parameters = request.query_string_parameters();
    let id = query_parameters.first("id");
    let email = query_parameters.first("amount");
    if id.is_none() && email.is_none() {
        return Err(response_error(
            400,
            "id or email parameter must be required",
        ));
    }

    if let Some(i) = id {
        let acct = dc
            .get_account_by_id_async(i)
            .await
            .map_err(|e| response_error(404, &format!("Failed to get account {}", e)))?;
        return Ok(Response::builder()
            .status(200)
            .body(serde_json::to_string(&acct)?.into())?);
    }

    if let Some(email) = email {
        let acct = dc
            .get_account_by_email_async(email)
            .await
            .map_err(|e| response_error(404, &format!("Failed to get account {}", e)))?;
        return Ok(Response::builder()
            .status(200)
            .body(serde_json::to_string(&acct)?.into())?);
    }

    Ok(Response::builder()
        .status(400)
        .body("no id or email query parameter were provided".into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(get_acct)).await
}
