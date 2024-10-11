use core::f64;
use std::sync::atomic::AtomicU64;

use crate::users::User;

pub struct Account {
    id: u64,
    user: User,
    balance: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AccountError {
    Overdraft,
    NegativeAmount,
    DepositError,
    AccountExists,
    DoesNotExist,
}

impl Account {
    pub fn new(user: User) -> Self {
        Account {
            id: Self::get_id(),
            user,
            balance: 0.0,
        }
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<f64, AccountError> {
        if amount <= 0.0 {
            return Err(AccountError::NegativeAmount);
        }

        let balance = self.balance;
        let remainder = balance - amount;
        if remainder < 0.0 {
            return Err(AccountError::Overdraft);
        }
        self.balance = remainder;
        Ok(self.balance)
    }

    pub fn deposit(&mut self, amount: f64) -> Result<f64, AccountError> {
        if amount <= 0.0 {
            return Err(AccountError::NegativeAmount);
        }

        self.balance += amount;
        Ok(self.balance)
    }

    fn get_id() -> u64 {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acct_creation() {
        let user = User::new("Test User".to_string(), "TestEmail@test.com".to_string());
        let acct = Account::new(user);
        assert_eq!(acct.balance, 0.0);

        let user2 = User::new("Test User2".to_string(), "TestEmail2@test.com".to_string());
        let acct2 = Account::new(user2);
        assert_ne!(acct.id, acct2.id);
    }

    #[test]
    fn test_acct_deposit_ok() {
        let user = User::new("Test User".to_string(), "TestEmail@test.com".to_string());
        let mut acct = Account::new(user);
        let res = acct.deposit(42.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), acct.balance);
        assert_eq!(acct.balance, 42.0);
    }

    #[test]
    fn test_acct_deposit_error() {
        let user = User::new("Test User".to_string(), "TestEmail@test.com".to_string());
        let mut acct = Account::new(user);
        let res = acct.deposit(-42.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), AccountError::NegativeAmount);
    }

    #[test]
    fn test_acct_withdraw_ok() {
        let user = User::new("Test User".to_string(), "TestEmail@test.com".to_string());
        let mut acct = Account::new(user);
        let dres = acct.deposit(42.0);
        let wres = acct.withdraw(24.0);
        assert!(dres.is_ok());
        assert!(wres.is_ok());
        assert_eq!(wres.unwrap(), acct.balance);
        assert_eq!(acct.balance, 18.0);
    }

    #[test]
    fn test_acct_overdraft_error() {
        let user = User::new("Test User".to_string(), "TestEmail@test.com".to_string());
        let mut acct = Account::new(user);
        let dres = acct.deposit(42.0);
        let wres = acct.withdraw(43.0);
        assert!(dres.is_ok());
        assert!(wres.is_err());
        assert_eq!(wres.unwrap_err(), AccountError::Overdraft);
    }

    #[test]
    fn test_acct_negative_withdraw_error() {
        let user = User::new("Test User".to_string(), "TestEmail@test.com".to_string());
        let mut acct = Account::new(user);
        let dres = acct.deposit(42.0);
        let wres = acct.withdraw(-43.0);
        assert!(dres.is_ok());
        assert!(wres.is_err());
        assert_eq!(wres.unwrap_err(), AccountError::NegativeAmount);
    }
}
