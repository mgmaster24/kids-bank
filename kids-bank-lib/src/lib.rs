mod acct_management;
mod users;

use async_trait::async_trait;

pub use acct_management::{Account, AccountError};

pub fn create_user_account(name: &str, email: &str) -> Result<Account, String> {
    let user = users::User::new(name.to_string(), email.to_string());
    Ok(acct_management::Account::new(user))
}

#[async_trait]
pub trait AccountHandler {
    async fn create_account(name: &str, email: &str) -> Result<Account, AccountError>;
    async fn get_accounts() -> Result<Vec<Account>, AccountError>;
    async fn get_account_by_id(id: u64) -> Result<Account, AccountError>;
    async fn get_account_by_email(email: &str) -> Result<Account, AccountError>;
    async fn withdraw(account_id: u64, amount: f64) -> Result<f64, AccountError>;
    async fn deposit(account_id: u64, amount: f64) -> Result<f64, AccountError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        create_user_account("some user", "someuser@email.com");
    }
}
