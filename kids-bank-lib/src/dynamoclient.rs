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

impl Clone for DynamoClient {
    fn clone(&self) -> Self {
        Self {
            table_name: self.table_name.clone(),
            client: self.client.clone(),
        }
    }
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
        let res = self
            .client
            .get_item()
            .table_name(self.table_name())
            .key(attr, attr_val.clone())
            .send()
            .await
            .map_err(|e| {
                AccountError::RetrievalError(format!(
                    "Failed to retrieve item for {attr} from {} for value {:?}. Reason: {:?}",
                    self.table_name(),
                    attr_val.clone(),
                    e
                ))
            })?;
        let attr_map = res.item().ok_or_else(|| {
            AccountError::RetrievalError(format!(
                "Item for {attr} with value {:?} not found in table {}",
                attr_val,
                self.table_name()
            ))
        })?;

        self.get_account_from_attributes(attr_map)
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

    fn get_account_from_attributes(
        &self,
        attr_map: &HashMap<String, AttributeValue>,
    ) -> Result<Account, AccountError> {
        let id = attr_map
            .get("id")
            .ok_or(AccountError::MissingAttribute("id".to_string()))?
            .as_s()
            .map_err(|_| AccountError::InvalidAttributeType("id".to_string()))?;

        let email = attr_map
            .get("email")
            .ok_or(AccountError::MissingAttribute("email".to_string()))?
            .as_s()
            .map_err(|_| AccountError::InvalidAttributeType("email".to_string()))?;

        let name = attr_map
            .get("name")
            .ok_or(AccountError::MissingAttribute("name".to_string()))?
            .as_s()
            .map_err(|_| AccountError::InvalidAttributeType("name".to_string()))?;

        let pw = attr_map
            .get("password")
            .ok_or(AccountError::MissingAttribute("password".to_string()))?
            .as_s()
            .map_err(|_| AccountError::InvalidAttributeType("password".to_string()))?;

        let balance = attr_map
            .get("balance")
            .ok_or(AccountError::MissingAttribute("balance".to_string()))?
            .as_n()
            .map_err(|_| AccountError::InvalidAttributeType("balance".to_string()))?
            .parse::<f64>()
            .map_err(|_| AccountError::InvalidBalanceFormat)?;
        Ok(create_acct_from_attributes(id, name, email, pw, balance))
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
            self.client
                .put_item()
                .table_name(self.table_name())
                .item("id", AttributeValue::S(account.id.to_string()))
                .item("email", AttributeValue::S(account.user.email().to_string()))
                .item("name", AttributeValue::S(account.user.name().to_string()))
                .item("password", AttributeValue::S(account.user.pw().to_string()))
                .item("balance", AttributeValue::N(account.balance.to_string()))
                .send()
                .await
                .map_err(|e| AccountError::CreationError(format!("{e:#}")))?;
            return Ok(account);
        }

        Err(AccountError::CreationError(
            "Failed to created account".to_string(),
        ))
    }

    async fn get_accounts_async(&self) -> Result<Vec<Account>, AccountError> {
        let res = self
            .client
            .scan()
            .table_name(self.table_name())
            .send()
            .await
            .map_err(|e| AccountError::RetrievalError(format!("{e:#}")))?;

        res.items.map_or(Ok(Vec::new()), |items| {
            let accounts: Result<Vec<_>, _> = items
                .into_iter()
                .map(|item| self.get_account_from_attributes(&item))
                .collect();
            accounts
        })
    }

    async fn get_account_by_id_async(&self, id: &str) -> Result<Account, AccountError> {
        self.get_account("id", AttributeValue::S(id.to_string()))
            .await
    }

    async fn get_account_by_email_async(&self, email: &str) -> Result<Account, AccountError> {
        let res = self
            .client
            .query()
            .table_name(self.table_name())
            .index_name("email-index")
            .key_condition_expression("#email = :email_value")
            .expression_attribute_names("#email", "email")
            .expression_attribute_values(":email_value", AttributeValue::S(email.to_string()))
            .send()
            .await
            .map_err(|e| AccountError::RetrievalError(format!("DynamoDB query error: {}", e)))?;
        let item = res
            .items
            .and_then(|items| items.into_iter().next())
            .ok_or(AccountError::DoesNotExist)?;

        self.get_account_from_attributes(&item)
    }

    async fn deposit_async(&self, account_id: &str, amount: f64) -> Result<f64, AccountError> {
        let mut acct = self
            .get_account_by_id_async(account_id)
            .await
            .map_err(|e| AccountError::RetrievalError(format!("{}", e)))?;

        let balance = acct.deposit(amount)?;
        self.update_balance(account_id, balance).await
    }

    async fn withdraw_async(&self, account_id: &str, amount: f64) -> Result<f64, AccountError> {
        let mut acct = self
            .get_account_by_id_async(account_id)
            .await
            .map_err(|e| AccountError::RetrievalError(format!("{}", e)))?;

        let balance = acct.withdraw(amount)?;
        self.update_balance(account_id, balance).await
    }
}
