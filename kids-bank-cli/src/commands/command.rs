use std::{collections::BTreeMap, process};

use crate::db::sqlite_db::Client;

use super::{cmd_create_account, cmd_get_accounts};

pub struct Command {
    name: String,
    description: String,
    pub callback: fn(client: &Client, args: &[String]),
}

pub fn get_commands() -> BTreeMap<String, Command> {
    let mut commands: BTreeMap<String, Command> = BTreeMap::new();
    commands.insert(
        "help".to_string(),
        Command {
            name: "help".to_string(),
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
            name: "exit".to_string(),
            description: String::from("Exits the Kids Bank application."),
            callback: |_: &Client, _: &[String]| {
                println!("Exiting Kids Bank");
                process::exit(0);
            },
        },
    );
    commands.insert(
        "get accounts".to_string(),
        Command {
            name: "get account".to_string(),
            description: String::from("Retrieves all the available account for Kids Bank."),
            callback: cmd_get_accounts::get_accounts,
        },
    );
    commands.insert(
        "create account".to_string(),
        Command {
            name: "create account".to_string(),
            description: String::from("Creates a Kids Bank account."),
            callback: cmd_create_account::create_account,
        },
    );
    commands
}
