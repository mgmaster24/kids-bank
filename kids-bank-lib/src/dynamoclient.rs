use crate::{
    create_acct_from_attributes, create_user_account, Account, AccountError, AsyncAccountHandler,
};
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use std::{collections::HashMap, f64};

#[derive(Debug)]
pub struct DynamoClient {
    table_name: String,
    client: aws_sdk_dynamodb::Client,
}

impl DynamoClient {
    pub fn new(config: &aws_config::SdkConfig, table_name: &str) -> Result<Self, String> {
        if table_name.is_empty() {
            return Err("Table name must not be empty".to_string());
        }

        let client = aws_sdk_dynamodb::Client::new(config);
        Ok(DynamoClient {
            table_name: table_name.to_string(),
            client,
        })
    }

    pub async fn create_new_account(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<Account, AccountError> {
        self.create_account_async(name, email, password).await
    }

    fn table_name(&self) -> String {
        self.table_name.to_string()
    }

    async fn get_account(
        &self,
        attr: &str,
        attr_val: AttributeValue,
    ) -> Result<Account, AccountError> {
        match &self
            .client
            .get_item()
            .table_name(self.table_name())
            .key(attr, attr_val.clone())
            .send()
            .await
        {
            Ok(res) => {
                let attr_map = res.item().expect("expected HashMap of AttributeValues");
                Ok(self.get_account_from_attributes(attr_map))
            }
            Err(e) => {
                let table_name = self.table_name();
                Err(AccountError::RetrievalError(format!(
                    "Failed to retrieve item for {attr} from {table_name} for value {:?}. Reason: {:?}",
                    attr_val.clone(), e
                )))
            }
        }
    }

    async fn update_balance(&self, id: &str, balance: f64) -> Result<f64, AccountError> {
        let request = &self
            .client
            .update_item()
            .table_name(self.table_name())
            .key("id", AttributeValue::S(id.to_string()))
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
            .as_s()
            .unwrap();
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
        let pw = attr_map
            .get("password")
            .expect("expected password value to exist")
            .as_s()
            .unwrap();
        let balance = attr_map
            .get("balance")
            .expect("expected balance value to exist")
            .as_n()
            .unwrap()
            .parse::<f64>()
            .expect("expect balance to be f64");
        create_acct_from_attributes(id, name, email, pw, balance)
    }
}

#[async_trait]
impl AsyncAccountHandler for DynamoClient {
    async fn create_account_async(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<Account, AccountError> {
        if (self.get_account_by_email_async(email).await).is_ok() {
            return Err(AccountError::AccountExists);
        }

        if let Ok(account) = create_user_account(name, email, password) {
            match &self
                .client
                .put_item()
                .table_name(self.table_name())
                .item("id", AttributeValue::S(account.id.to_string()))
                .item("email", AttributeValue::S(account.user.email().to_string()))
                .item("name", AttributeValue::S(account.user.name().to_string()))
                .item("password", AttributeValue::S(account.user.pw().to_string()))
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

    async fn get_accounts_async(&self) -> Result<Vec<Account>, AccountError> {
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
                if let Some(items) = &res.items {
                    for i in items {
                        let account = self.get_account_from_attributes(i);
                        accounts.push(account);
                    }
                }
                return Ok(accounts);
            }
            Err(e) => return Err(AccountError::RetrievalError(format!("{e:#}"))),
        }
    }

    async fn get_account_by_id_async(&self, id: &str) -> Result<Account, AccountError> {
        self.get_account("id", AttributeValue::S(id.to_string()))
            .await
    }

    async fn get_account_by_email_async(&self, email: &str) -> Result<Account, AccountError> {
        match &self
            .client
            .query()
            .table_name(self.table_name())
            .index_name("email-index")
            .key_condition_expression("#email = :email_value")
            .expression_attribute_names("#email", "email")
            .expression_attribute_values(":email_value", AttributeValue::S(email.to_string()))
            .send()
            .await
        {
            Ok(res) => {
                if let Some(items) = &res.items {
                    if let Some(item) = items.iter().next() {
                        Ok(self.get_account_from_attributes(item))
                    } else {
                        Err(AccountError::DoesNotExist)
                    }
                } else {
                    Err(AccountError::DoesNotExist)
                }
            }
            Err(err) => Err(AccountError::RetrievalError(format!(
                "DynamoDB query error: {}",
                err
            ))),
        }
    }

    async fn deposit_async(&self, account_id: &str, amount: f64) -> Result<f64, AccountError> {
        let acct_res = self.get_account_by_id_async(account_id).await;
        if let Err(e) = acct_res {
            return Err(AccountError::RetrievalError(format!("{}", e)));
        }

        let mut acct = acct_res.unwrap();
        let dep_res = acct.deposit(amount);
        match dep_res {
            Ok(balance) => self.update_balance(account_id, balance).await,
            Err(e) => return Err(e),
        }
    }

    async fn withdraw_async(&self, account_id: &str, amount: f64) -> Result<f64, AccountError> {
        let acct_res = self.get_account_by_id_async(account_id).await;
        if let Err(e) = acct_res {
            return Err(AccountError::RetrievalError(format!("{}", e)));
        }

        let mut acct = acct_res.unwrap();
        let wd_res = acct.withdraw(amount);
        match wd_res {
            Ok(balance) => self.update_balance(account_id, balance).await,
            Err(e) => Err(e),
        }
    }
}
