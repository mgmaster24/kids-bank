use crate::commands::{create_account, deposit, get_account, get_accounts, withdraw};
use crate::db::Client;
use std::{collections::BTreeMap, process};

pub struct Command {
    description: String,
    callback: fn(client: &Client, args: &[String]),
}

impl Command {
    pub fn call(&self, client: &Client, args: &[String]) {
        (self.callback)(client, args);
    }
}

pub fn get_commands() -> BTreeMap<String, Command> {
    let mut commands: BTreeMap<String, Command> = BTreeMap::new();
    commands.insert(
        "help".to_string(),
        Command {
            description: String::from("Describes the available commands for Kids Bank."),
            callback: |_: &Client, _: &[String]| {
                let commands = get_commands();
                for (k, v) in commands {
                    println!("{} - {}", k, v.description);
                }
            },
        },
    );
    commands.insert(
        "exit".to_string(),
        Command {
            description: String::from("Exits the Kids Bank application."),
            callback: |_: &Client, _: &[String]| {
                println!("Exiting Kids Bank");
                process::exit(0);
            },
        },
    );
    commands.insert(
        "list".to_string(),
        Command {
            description: String::from("Retrieves all the available account for Kids Bank."),
            callback: get_accounts,
        },
    );
    commands.insert(
        "create".to_string(),
        Command {
            description: String::from("Creates a Kids Bank account."),
            callback: create_account,
        },
    );
    commands.insert(
        "retrieve".to_string(),
        Command {
            description: String::from("Get an account by email."),
            callback: get_account,
        },
    );
    commands.insert(
        "deposit".to_string(),
        Command {
            description: String::from("Deposit amount to an account."),
            callback: deposit,
        },
    );
    commands.insert(
        "withdraw".to_string(),
        Command {
            description: String::from("Withdraw from and account."),
            callback: withdraw,
        },
    );

    commands
}
