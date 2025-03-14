use core::f64;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::users::User;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub user: User,
    pub balance: f64,
    pub current_apr: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AccountError {
    CreationError(String),
    RetrievalError(String),
    BalanceError(String),
    MissingAttribute(String),
    InvalidAttributeType(String),
    InvalidBalanceFormat,
    Overdraft,
    NegativeAmount,
    DepositError,
    AccountExists,
    DoesNotExist,
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountError::CreationError(s) => {
                let err_msg = format!("Failed to create account! Reason: {}", s);
                f.write_str(&err_msg)
            }
            AccountError::RetrievalError(s) => {
                let err_msg = format!("Failed to get account! Reason: {}", s);
                f.write_str(&err_msg)
            }
            AccountError::BalanceError(s) => {
                let err_msg = format!("Failed to update balance for account! Reason: {}", s);
                f.write_str(&err_msg)
            }
            AccountError::MissingAttribute(s) => {
                let err_msg = format!(
                    "Failed to create account from returned attributes.  {} is missing",
                    s
                );
                f.write_str(&err_msg)
            }
            AccountError::InvalidAttributeType(s) => {
                let err_msg = format!(
                    "Failed to create account from returned attributes.  {} is invalid",
                    s
                );
                f.write_str(&err_msg)
            }
            AccountError::InvalidBalanceFormat => {
                write!(f, "Float is expected for balance.")
            }
            AccountError::Overdraft => {
                write!(f, "Cannot withdraw funds! Account would be overdrafted.")
            }
            AccountError::NegativeAmount => {
                write!(f, "Cannot not apply a negative amount to the balance.")
            }
            AccountError::DepositError => {
                write!(f, "An error occurred during the depositing of funds")
            }
            AccountError::AccountExists => write!(
                f,
                "Failed to create account! Account already exists for provided email"
            ),
            AccountError::DoesNotExist => write!(f, "Account does NOT exist!"),
        }
    }
}

impl Account {
    pub fn new(user: User) -> Self {
        let words: Vec<_> = user.name().split_whitespace().collect();
        let id = words.join("");
        Account {
            id: id.to_owned() + "_" + user.email().as_str(),
            user,
            balance: 0.0,
            current_apr: 0.05,
        }
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<f64, AccountError> {
        if amount <= 0.0 {
            return Err(AccountError::NegativeAmount);
        }

        let remainder = self.balance - amount;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acct_creation() {
        let user = User::new("Test User", "TestEmail@test.com", "some_pass_word");
        let acct = Account::new(user);
        assert_eq!(acct.balance, 0.0);

        let user2 = User::new("Test User2", "TestEmail2@test.com", "some_pass_word");
        let acct2 = Account::new(user2);
        assert_ne!(acct.id, acct2.id);
    }

    #[test]
    fn test_acct_deposit_ok() {
        let user = User::new("Test User", "TestEmail@test.com", "some_pass_word");
        let mut acct = Account::new(user);
        let res = acct.deposit(42.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), acct.balance);
        assert_eq!(acct.balance, 42.0);
    }

    #[test]
    fn test_acct_deposit_error() {
        let user = User::new("Test User", "TestEmail@test.com", "some_pass_word");
        let mut acct = Account::new(user);
        let res = acct.deposit(-42.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), AccountError::NegativeAmount);
    }

    #[test]
    fn test_acct_withdraw_ok() {
        let user = User::new("Test User", "TestEmail@test.com", "some_pass_word");
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
        let user = User::new("Test User", "TestEmail@test.com", "some_pass_word");
        let mut acct = Account::new(user);
        let dres = acct.deposit(42.0);
        let wres = acct.withdraw(43.0);
        assert!(dres.is_ok());
        assert!(wres.is_err());
        assert_eq!(wres.unwrap_err(), AccountError::Overdraft);
    }

    #[test]
    fn test_acct_negative_withdraw_error() {
        let user = User::new("Test User", "TestEmail@test.com", "some_pass_word");
        let mut acct = Account::new(user);
        let dres = acct.deposit(42.0);
        let wres = acct.withdraw(-43.0);
        assert!(dres.is_ok());
        assert!(wres.is_err());
        assert_eq!(wres.unwrap_err(), AccountError::NegativeAmount);
    }
}
