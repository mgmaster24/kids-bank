use crate::db::Client;
use kids_bank_lib::AccountHandler;

pub fn get_account(client: &Client, args: &[String]) {
    if args.is_empty() || args.len() > 1 {
        println!("Too many arguments for this method");
        return;
    }

    let email = args[0].as_str();
    let account_res = client.get_account_by_email(email);
    match account_res {
        Ok(account) => {
            println!("\t Account Holder: {}", account.user.name());
            println!("\t Balance: {}", account.balance);
        }
        Err(e) => println!("{}", e),
    }
}
