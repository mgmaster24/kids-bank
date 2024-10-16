mod acct_management;
mod users;

use async_trait::async_trait;

pub use acct_management::{Account, AccountError};

pub fn create_user_account(name: &str, email: &str) -> Result<Account, String> {
    let user = users::User::new(name, email);
    Ok(acct_management::Account::new(user))
}

pub fn create_account(id: &str, name: &str, email: &str, balance: f64) -> Account {
    Account {
        id: id.to_string(),
        user: users::User::new(name, email),
        balance,
    }
}

#[async_trait]
pub trait AccountHandler {
    async fn create_account(&self, name: &str, email: &str) -> Result<Account, AccountError>;
    async fn get_accounts(&self) -> Result<Vec<Account>, AccountError>;
    async fn get_account_by_id(&self, id: &str) -> Result<Account, AccountError>;
    async fn get_account_by_email(&self, email: &str) -> Result<Account, AccountError>;
    async fn withdraw(&self, account_id: &str, amount: f64) -> Result<f64, AccountError>;
    async fn deposit(&self, account_id: &str, amount: f64) -> Result<f64, AccountError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = create_user_account("some user", "someuser@email.com");
        assert!(res.is_ok());
    }
}
