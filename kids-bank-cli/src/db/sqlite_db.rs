use kids_bank_lib::{create_account, Account, AccountError, AccountHandler};
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
                        balance REAL DEFAULT 0.0 NOT NULL
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
}

impl AccountHandler for Client {
    fn create_account(&self, name: &str, email: &str) -> Result<bool, kids_bank_lib::AccountError> {
        let result = self.connection.execute(
            "INSERT INTO accounts (name, email) VALUES (?1, ?2)",
            (name.to_string(), email.to_string()),
        );

        match result {
            Ok(_) => Ok(true),
            Err(e) => Err(kids_bank_lib::AccountError::CreationError(format!("{}", e))),
        }
    }

    fn get_accounts(&self) -> Result<Vec<Account>, kids_bank_lib::AccountError> {
        let stat_res = self.connection.prepare("SELECT * FROM accounts");
        match stat_res {
            Ok(mut stat) => {
                let accounts_iter = stat.query_map([], |row| {
                    let id: i32 = row.get(0)?;
                    let name: String = row.get(1)?;
                    let email: String = row.get(2)?;
                    let balance: f64 = row.get(3)?;

                    Ok(create_account(
                        id.to_string().as_str(),
                        name.as_str(),
                        email.as_str(),
                        balance,
                    ))
                });

                let accounts_iter = match accounts_iter {
                    Ok(ai) => ai,
                    Err(e) => return Err(AccountError::RetrievalError(e.to_string())),
                };

                let mut accounts = Vec::new();
                for acct in accounts_iter {
                    accounts.push(acct.expect("Should be an account"));
                }
                Ok(accounts)
            }
            Err(e) => Err(AccountError::RetrievalError(e.to_string())),
        }
    }

    fn get_account_by_id(&self, id: &str) -> Result<Account, kids_bank_lib::AccountError> {
        let stat_res = self.connection.prepare("SELECT id FROM accounts WHERE id =  ")
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
