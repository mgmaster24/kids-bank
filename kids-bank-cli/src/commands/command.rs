use std::{collections::BTreeMap, process};

use crate::db::sqlite_db::Client;

use super::{
    cmd_create_account, cmd_deposit, cmd_get_account_by_email, cmd_get_accounts, cmd_withdraw,
};

pub struct Command {
    description: String,
    pub callback: fn(client: &Client, args: &[String]),
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
            callback: cmd_get_accounts::get_accounts,
        },
    );
    commands.insert(
        "create".to_string(),
        Command {
            description: String::from("Creates a Kids Bank account."),
            callback: cmd_create_account::create_account,
        },
    );
    commands.insert(
        "retrieve".to_string(),
        Command {
            description: String::from("Get an account by email."),
            callback: cmd_get_account_by_email::get_account,
        },
    );
    commands.insert(
        "deposit".to_string(),
        Command {
            description: String::from("Deposit amount to an account."),
            callback: cmd_deposit::deposit,
        },
    );
    commands.insert(
        "withdraw".to_string(),
        Command {
            description: String::from("Withdraw from and account."),
            callback: cmd_withdraw::withdraw,
        },
    );

    commands
}
