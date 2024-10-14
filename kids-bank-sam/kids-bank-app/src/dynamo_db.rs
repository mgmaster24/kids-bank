use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use kids_bank_lib::{create_account, create_user_account, Account, AccountError, AccountHandler};
use std::{collections::HashMap, f64, hash::RandomState, u64};

#[derive(Debug)]
struct DynamoClient {
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

    pub fn table_name(&self) -> String {
        self.table_name.to_string()
    }
}

#[async_trait]
impl AccountHandler for DynamoClient {
    async fn create_account(&self, name: &str, email: &str) -> Result<Account, AccountError> {
        // get account by provided email

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
                let accounts: Vec<Account> = Vec::new();
                //res.
                return Ok(accounts);
            }
            Err(e) => return Err(AccountError::RetrievalError(format!("{e:#}"))),
        }
    }

    async fn get_account_by_id(&self, id: u64) -> Result<Account, AccountError> {
        match &self
            .client
            .get_item()
            .table_name(self.table_name())
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await
        {
            Ok(res) => {
                let attr_map = res.item().expect("expected HashMap of AttributeValues");
                return Ok(get_account_from_attributes(attr_map));
            }
            Err(_) => {
                return Err(AccountError::RetrievalError(format!(
                    "Failed to retrieve item for {id}"
                )))
            }
        }
    }

    async fn get_account_by_email(&self, email: &str) -> Result<Account, AccountError> {
        Err(AccountError::DoesNotExist)
    }

    async fn deposit(&self, account_id: u64, amount: f64) -> Result<f64, AccountError> {
        Err(AccountError::DepositError)
    }

    async fn withdraw(&self, account_id: u64, amount: f64) -> Result<f64, AccountError> {
        Err(AccountError::NegativeAmount)
    }
}

fn get_account_from_attributes(attr_map: &HashMap<String, AttributeValue>) -> Account {
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
