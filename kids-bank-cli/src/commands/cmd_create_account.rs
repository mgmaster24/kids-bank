use crate::db::Client;
use kids_bank_lib::AccountHandler;
use std::io::{self, Write};

pub fn create_account(client: &Client, args: &[String]) {
    if !args.is_empty() {
        println!("Invalid number of arguments to create account command");
        return;
    }

    let mut name = String::new();
    let mut email = String::new();
    loop {
        print!("Enter user name: ");
        io::stdout().flush().expect("flush failed");
        std::io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        if name.is_empty() {
            println!("You MUST provide a name");
            continue;
        }

        print!("Enter user email: ");
        io::stdout().flush().expect("flush failed");
        std::io::stdin()
            .read_line(&mut email)
            .expect("Failed to read line");

        if email.is_empty() {
            println!("You MUST provide an email");
        }

        if !name.is_empty() && !email.is_empty() {
            break;
        }
    }
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
