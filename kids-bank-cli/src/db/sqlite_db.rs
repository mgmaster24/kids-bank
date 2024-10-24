use kids_bank_lib::{create_account, Account, AccountError, AccountHandler};
use rusqlite;

pub struct Client {
    pub connection: rusqlite::Connection,
}

impl Client {
    pub fn open(path: &str) -> Result<Client, rusqlite::Error> {
        match rusqlite::Connection::open(path) {
            Ok(c) => Ok(Client { connection: c }),
            Err(e) => Err(e),
        }
    }

    pub fn create_table(&self) -> Result<bool, rusqlite::Error> {
        let mut stmt = self.connection.prepare(
            "SELECT name FROM sqlite_master 
            WHERE type='table' AND name=:accounts",
        )?;
        let rows = stmt.query_map(&[(":accounts", "accounts")], |_| Ok(()))?;
        if rows.count() == 1 {
            return Ok(true);
        }

        let result = self.connection.execute(
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

        Ok(true)
    }

    fn select_where(&self, col: &str, val: &str) -> Result<Account, AccountError> {
        let param = format!(":{col}");
        let statement = format!("SELECT * FROM account WHERE {col}={param}");
        let stat_res = self.connection.prepare(statement.as_str());
        match stat_res {
            Ok(mut stat) => {
                let account = stat.query_row(&[(param.as_str(), val)], |row| {
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

                match account {
                    Ok(a) => Ok(a),
                    Err(e) => Err(AccountError::RetrievalError(e.to_string())),
                }
            }
            Err(e) => Err(AccountError::RetrievalError(e.to_string())),
        }
    }

    fn update_balance(&self, balance: f64, account_id: &str) -> Result<f64, AccountError> {
        let statement_res = self
            .connection
            .prepare("UPDATE accounts SET balance=:balance WHERE id=:id");
        match statement_res {
            Ok(mut statement) => {
                let update_res = statement.execute(&[
                    (":balance", balance.to_string().as_str()),
                    (":id", account_id),
                ]);
                if update_res.is_ok() {
                    return Ok(balance);
                }

                Err(AccountError::BalanceError(
                    "Failed to execute the update statement".to_string(),
                ))
            }
            Err(e) => Err(AccountError::BalanceError(e.to_string())),
        }
    }
}

impl AccountHandler for Client {
    fn create_account(&self, name: &str, email: &str) -> Result<bool, kids_bank_lib::AccountError> {
        let result = self.connection.execute(
            "INSERT INTO accounts (name, email, balance) VALUES (?1, ?2, ?3)",
            (name.to_string(), email.to_string(), 0.0),
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

                let mut accounts: Vec<Account> = Vec::new();
                for acct in accounts_iter {
                    accounts.push(acct.expect("Should be an account"));
                }
                Ok(accounts)
            }
            Err(e) => Err(AccountError::RetrievalError(e.to_string())),
        }
    }

    fn get_account_by_id(&self, id: &str) -> Result<Account, kids_bank_lib::AccountError> {
        self.select_where("id", id)
    }

    fn get_account_by_email(&self, email: &str) -> Result<Account, kids_bank_lib::AccountError> {
        self.select_where("email", email)
    }

    fn deposit(&self, account_id: &str, amount: f64) -> Result<f64, kids_bank_lib::AccountError> {
        match self.get_account_by_id(account_id) {
            Ok(mut account) => {
                let res = account.deposit(amount);
                match res {
                    Ok(balance) => {
                        let update_res = self.update_balance(balance, account_id);
                        if update_res.is_ok() {
                            return res;
                        }

                        Err(AccountError::BalanceError(
                            update_res.expect_err("Should be error").to_string(),
                        ))
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn withdraw(&self, account_id: &str, amount: f64) -> Result<f64, kids_bank_lib::AccountError> {
        match self.get_account_by_id(account_id) {
            Ok(mut account) => {
                let res = account.withdraw(amount);
                match res {
                    Ok(balance) => {
                        let update_res = self.update_balance(balance, account_id);
                        if update_res.is_ok() {
                            return res;
                        }

                        Err(AccountError::BalanceError(
                            update_res.expect_err("Should be an error").to_string(),
                        ))
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}
