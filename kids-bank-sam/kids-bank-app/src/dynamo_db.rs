use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use kids_bank_lib::{self, Account, AccountError, AccountHandler};

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
}

#[async_trait]
impl AccountHandler for DynamoClient {
    async fn create_account(&self, name: &str, email: &str) -> Result<Account, AccountError> {
        if let Ok(account) = kids_bank_lib::create_user_account(name, email) {
            let res = self
                .client
                .put_item()
                .table_name(self.table_name.to_owned())
                .item("id", AttributeValue::S(account.id.to_string()))
                .item("email", AttributeValue::S(account.user.email().to_string()))
                .item("name", AttributeValue::S(account.user.name().to_string()))
                .send()
                .await;

            return Ok(account);
        }

        Err(AccountError::CreationError)
    }

    async fn get_accounts(&self) -> Result<Vec<Account>, AccountError> {
        Err(AccountError::DoesNotExist)
    }

    async fn get_account_by_id(&self, id: u64) -> Result<Account, AccountError> {
        Err(AccountError::DoesNotExist)
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
