use kids_bank_lib::{AsyncAccountHandler, DynamoClient};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::Value;
use std::env;

async fn update_account_balances(_event: LambdaEvent<Value>) -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dc_res = DynamoClient::new(&config, &table_name);
    if dc_res.is_err() {
        return Err("Couldn't create DynamoClient".into());
    }

    let dc = dc_res.unwrap();
    let accts_res = dc.get_accounts_async().await;
    match accts_res {
        Ok(accts) => {
            for ele in accts {
                let amount: f64 = ele.balance * ele.current_apr;
                let acct_res = dc.deposit_async(&ele.id, amount).await;
                if acct_res.is_err() {
                    return Err(format!("Couldn't update balance for account {}", &ele.id).into());
                }
            }

            Ok(())
        }
        Err(e) => Err(format!("Failed to get accounts {}", e).into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(update_account_balances)).await
}
