use kids_bank_lib::{self, Account, AccountError, AccountHandler};
use lambda_http::{http::Error, Body, Response};

#[derive(Debug)]
struct DynamoClient {
    table_name: String,
    client: aws_sdk_dynamodb::Client,
}


impl DynamoClient {
    pub fn new(config: &aws_config::SdkConfig, table_name: String) -> Result<Self, String> {
        let client = aws_sdk_dynamodb::Client::new(config);

        Ok(DynamoClient { table_name, client })
    }

    pub async fn get_item(&self) -> Result<Response<Body>, Error> {
        Err()
    }

    async pub fn put_item(&self, request: lambda_http::Request) -> Result<Response<Body>, Error> {
        Err(Error("something happened"))
    }
}

impl AccountHandler for DynamoClient {
    pub fn create_account(name: &str, email: &str) -> Result<Account, AccountError> {
        let account = kids_bank_lib::create_user_account(name, email);
        // write account to dynamodb
    }

    pub fn get_accounts() -> Result<Vec<Account>, AccountError> {
        Err(AccountError::DoesNotExist)
    }

    pub fn get_account_by_id(id: u64) -> Result<Account, AccountError> {}

    pub fn get_account_by_email(email: &str) -> Result<Account, AccountError> {}

    pub fn deposit(account_id: u64, amount: f64) -> Result<f64, AccountError> {}

    pub fn withdraw(account_id: u64, amount: f64) -> Result<f64, AccountError> {}
}
