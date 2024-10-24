use kids_bank_lib::AccountHandler;

use crate::db::sqlite_db::Client;

pub fn create_account(client: &Client, args: &[String]) {
    if args.is_empty() && args.len() > 2 {
        println!("Invalid number of arguments to create account command");
        return;
    }

    let name = args[0].to_string();
    let email = args[1].to_string();
    let results = client.create_account(name.as_str(), email.as_str());
    match results {
        Ok(_) => {
            println!("Created account for {} using email {}", name, email);
        }
        Err(e) => println!(
            "Error occurred when attempting to create an account. Error {}",
            e
        ),
    }
}
