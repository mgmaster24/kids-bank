use crate::db::sqlite_db::Client;
use kids_bank_lib::AccountHandler;

pub fn get_accounts(client: &Client, args: &[String]) {
    if !args.is_empty() {
        println!("No arguments necessary for this method");
        return;
    }

    let accounts_res = client.get_accounts();
    match accounts_res {
        Ok(accounts) => {
            for account in accounts {
                println!("/t Account Holder:{}", account.user.name());
                println!("/t Balance: {}", account.balance);
            }
        }
        Err(e) => println!("{}", e),
    }
}
