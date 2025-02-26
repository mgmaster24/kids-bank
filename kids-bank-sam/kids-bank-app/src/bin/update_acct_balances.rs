use kids_bank_lib::{AsyncAccountHandler, DynamoClient};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::Value;
use std::env;

async fn update_account_balances(_event: LambdaEvent<Value>) -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").map_err(|_| "TABLE_NAME must be set")?;
    let dc =
        DynamoClient::new(&config, &table_name).map_err(|_| "Failed to create DynamoClient")?;
    let accts = dc
        .get_accounts_async()
        .await
        .map_err(|e| format!("Failed to get accounts {}", e))?;
    for ele in &accts {
        if let Err(e) = dc
            .deposit_async(&ele.id, ele.balance * ele.current_apr)
            .await
        {
            return Err(format!("Couldn't update balance for account {}: {}", ele.id, e).into());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(update_account_balances)).await
}
