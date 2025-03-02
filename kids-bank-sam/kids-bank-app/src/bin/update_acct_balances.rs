use futures::future::join_all;
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
    let futures = accts.iter().map(|acct| {
        let dc_ref = dc.clone();
        async move {
            dc_ref
                .deposit_async(&acct.id, acct.current_apr * acct.balance)
                .await
                .map_err(|e| format!("Couldn't update balance for account {}: {}", acct.id, e))
        }
    });

    let results = join_all(futures).await;
    let errors: Vec<String> = results.into_iter().filter_map(Result::err).collect();
    if !errors.is_empty() {
        eprintln!("Failed to update balance for the following accounts:");
        for e in errors {
            eprintln!("{}", e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(update_account_balances)).await
}
