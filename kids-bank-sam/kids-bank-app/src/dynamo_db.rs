use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use kids_bank_lib::{create_account, create_user_account, Account, AccountError, AccountHandler};
use std::{collections::HashMap, f64, u64};

#[derive(Debug)]
pub struct DynamoClient {
    table_name: String,
    client: aws_sdk_dynamodb::Client,
}

impl DynamoClient {
    pub fn new(config: &aws_config::SdkConfig, table_name: &str) -> Result<Self, String> {
        let client = aws_sdk_dynamodb::Client::new(config);

        Ok(DynamoClient {
            table_name: table_name.to_string(),
            client,
        })
    }

    fn table_name(&self) -> String {
        self.table_name.to_string()
    }

    async fn get_item(
        &self,
        attr: &str,
        attr_val: AttributeValue,
    ) -> Result<Account, AccountError> {
        match &self
            .client
            .get_item()
            .table_name(self.table_name())
            .key(attr, attr_val)
            .send()
            .await
        {
            Ok(res) => {
                let attr_map = res.item().expect("expected HashMap of AttributeValues");
                return Ok(self.get_account_from_attributes(attr_map));
            }
            Err(_) => {
                return Err(AccountError::RetrievalError(format!(
                    "Failed to retrieve item for {attr}"
                )))
            }
        }
    }

    async fn update_item(&self, id: u64, balance: f64) -> Result<f64, AccountError> {
        let request = &self
            .client
            .update_item()
            .table_name(self.table_name())
            .key("id", AttributeValue::N(id.to_string()))
            .update_expression("set balance = :balance")
            .expression_attribute_values(":balance", AttributeValue::N(balance.to_string()))
            .send()
            .await;

        match request {
            Ok(_) => Ok(balance),
            Err(_) => Err(AccountError::RetrievalError(
                "Failed to update balance".to_string(),
            )),
        }
    }

    fn get_account_from_attributes(&self, attr_map: &HashMap<String, AttributeValue>) -> Account {
        let id = attr_map
            .get("id")
            .expect("expected id value to exist")
            .as_n()
            .unwrap()
            .parse::<u64>()
            .expect("expect id to be u64");
        let email = attr_map
            .get("email")
            .expect("expected email value to exist")
            .as_s()
            .unwrap();
        let name = attr_map
            .get("name")
            .expect("expected name value to exist")
            .as_s()
            .unwrap();
        let balance = attr_map
            .get("balance")
            .expect("expected balance value to exist")
            .as_n()
            .unwrap()
            .parse::<f64>()
            .expect("expect balance to be f64");
        create_account(id, name, email, balance)
    }
}

#[async_trait]
impl AccountHandler for DynamoClient {
    async fn create_account(&self, name: &str, email: &str) -> Result<Account, AccountError> {
        // get account by provided email
        let acct_res = self.get_account_by_email(email).await;
        match acct_res {
            Ok(_) => return Err(AccountError::AccountExists),
            Err(_) => (),
        }

        if let Ok(account) = create_user_account(name, email) {
            match &self
                .client
                .put_item()
                .table_name(&self.table_name())
                .item("id", AttributeValue::S(account.id.to_string()))
                .item("email", AttributeValue::S(account.user.email().to_string()))
                .item("name", AttributeValue::S(account.user.name().to_string()))
                .item("balance", AttributeValue::N(account.balance.to_string()))
                .send()
                .await
            {
                Ok(_) => return Ok(account),
                Err(e) => return Err(AccountError::CreationError(format!("{e:#}"))),
            }
        }

        Err(AccountError::CreationError(
            "Failed to created account".to_string(),
        ))
    }

    async fn get_accounts(&self) -> Result<Vec<Account>, AccountError> {
        match &self
            .client
            .scan()
            .table_name(self.table_name())
            .send()
            .await
        {
            Ok(res) => {
                // get vector of accounts
                let mut accounts: Vec<Account> = Vec::new();
                if let Some(items) = res.items {}
                return Ok(accounts);
            }
            Err(e) => return Err(AccountError::RetrievalError(format!("{e:#}"))),
        }
    }

    async fn get_account_by_id(&self, id: u64) -> Result<Account, AccountError> {
        self.get_item("id", AttributeValue::N(id.to_string())).await
    }

    async fn get_account_by_email(&self, email: &str) -> Result<Account, AccountError> {
        self.get_item("email", AttributeValue::S(email.to_string()))
            .await
    }

    async fn deposit(&self, account_id: u64, amount: f64) -> Result<f64, AccountError> {
        if let Ok(mut acct) = self.get_account_by_id(account_id).await {
            let dep_res = acct.deposit(amount);
            match dep_res {
                Ok(balance) => {
                    self.update_item(account_id, balance);
                    return Ok(balance);
                }
                Err(e) => return Err(e),
            }
        }

        Err(AccountError::DepositError)
    }

    async fn withdraw(&self, account_id: u64, amount: f64) -> Result<f64, AccountError> {
        if let Ok(mut acct) = self.get_account_by_id(account_id).await {
            let wd_res = acct.withdraw(amount);
            match wd_res {
                Ok(balance) => {
                    self.update_item(account_id, balance);
                    return Ok(balance);
                }
                Err(e) => return Err(e),
            }
        }

        Err(AccountError::DoesNotExist)
    }
}
