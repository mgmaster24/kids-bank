pub mod accounts;
pub mod dynamo_client;
pub mod users;
use crate::accounts::{Account, AccountError};

use async_trait::async_trait;

pub fn create_user_account(name: &str, email: &str, pw: &str) -> Result<Account, String> {
    let user = users::User::new(name, email, pw);
    Ok(Account::new(user))
}

pub fn create_account(id: &str, name: &str, email: &str, pw: &str, balance: f64) -> Account {
    Account {
        id: id.to_string(),
        user: users::User::new(name, email, pw),
        balance,
    }
}

#[async_trait]
pub trait AsyncAccountHandler {
    async fn create_account_async(
        &self,
        name: &str,
        email: &str,
        pw: &str,
    ) -> Result<Account, AccountError>;
    async fn get_accounts_async(&self) -> Result<Vec<Account>, AccountError>;
    async fn get_account_by_id_async(&self, id: &str) -> Result<Account, AccountError>;
    async fn get_account_by_email_async(&self, email: &str) -> Result<Account, AccountError>;
    async fn withdraw_async(&self, account_id: &str, amount: f64) -> Result<f64, AccountError>;
    async fn deposit_async(&self, account_id: &str, amount: f64) -> Result<f64, AccountError>;
}

pub trait AccountHandler {
    fn create_account(&self, name: &str, email: &str) -> Result<bool, AccountError>;
    fn get_accounts(&self) -> Result<Vec<Account>, AccountError>;
    fn get_account_by_id(&self, id: &str) -> Result<Account, AccountError>;
    fn get_account_by_email(&self, email: &str) -> Result<Account, AccountError>;
    fn withdraw(&self, account_id: &str, amount: f64) -> Result<f64, AccountError>;
    fn deposit(&self, account_id: &str, amount: f64) -> Result<f64, AccountError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = create_user_account("some user", "someuser@email.com", "some_pw");
        assert!(res.is_ok());
    }
}
