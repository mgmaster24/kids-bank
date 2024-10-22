use kids_bank_lib::{create_account, Account, AccountHandler};
use rusqlite;

pub struct Client {
    pub connection: rusqlite::Connection,
}

impl Client {
    pub fn init(path: &str) -> Result<Client, rusqlite::Error> {
        match rusqlite::Connection::open(path) {
            Ok(c) => {
                let result = c.execute(
                    "CREATE TABLE accounts (
                        id INTERGER PRIMARY KEY,
                        name TEXT NOT NULL,
                        email TEXT NOT NULL,
                        balance INTEGER
                    )",
                    (),
                );

                if result.is_err() {
                    return Err(result.err().unwrap());
                }

                Ok(Client { connection: c })
            }
            Err(e) => Err(e),
        }
    }

    // pub fn get_all_accounts(&self) -> Result<Vec<Account>, AccountError> {
    //     let mut stat = &self.connection.prepare("SELECT * FROM accounts")?;
    //     let accounts_iter = stat.query_map((), |row| {
    //         let
    //         Ok(create_account(
    //             row.get(0)?,
    //             row.get(1)?,
    //             row.get(2)?,
    //             row.get(3)?,
    //         ))
    //     })?;
    // }
}

impl AccountHandler for Client {
    fn create_account(
        &self,
        name: &str,
        email: &str,
    ) -> Result<Account, kids_bank_lib::AccountError> {
        Err(kids_bank_lib::AccountError::DoesNotExist)
    }

    fn get_accounts(&self) -> Result<Vec<Account>, kids_bank_lib::AccountError> {
        Err(kids_bank_lib::AccountError::DoesNotExist)
    }

    fn get_account_by_id(&self, id: &str) -> Result<Account, kids_bank_lib::AccountError> {
        Err(kids_bank_lib::AccountError::DoesNotExist)
    }

    fn get_account_by_email(&self, email: &str) -> Result<Account, kids_bank_lib::AccountError> {
        Err(kids_bank_lib::AccountError::DoesNotExist)
    }

    fn deposit(&self, account_id: &str, amount: f64) -> Result<f64, kids_bank_lib::AccountError> {
        Err(kids_bank_lib::AccountError::DoesNotExist)
    }

    fn withdraw(&self, account_id: &str, amount: f64) -> Result<f64, kids_bank_lib::AccountError> {
        Err(kids_bank_lib::AccountError::DoesNotExist)
    }
}
