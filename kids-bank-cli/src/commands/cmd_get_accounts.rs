use kids_bank_lib::{Account, AccountError};

use crate::db::sqlite_db::Client;

pub fn get_accounts(client: &Client, args: &[String]) {
    //Err(AccountError::RetrievalError(
    //    "failed to get account".to_string(),
    //))
}
