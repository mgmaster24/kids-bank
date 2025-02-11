use std::io::{self, Write};

use crate::{commands::get_commands, db::Client};

pub fn start_repl(client: &Client) {
    println!("Welcome to Kids Bank");
    println!("Run \"help\" command to see available acitons");
    loop {
        print!("Kids Bank -> ");
        io::stdout().flush().expect("flush failed");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.is_empty() {
            continue;
        }

        let inputs = clean_input(input.as_str());
        let command_name = &inputs[0];
        let args = &inputs[1..];
        let cmds = get_commands();
        let cmd = cmds.get(command_name);
        match cmd {
            Some(c) => c.call(client, args),
            None => {
                println!("No command with that name. Try again. Type help for more information.");
            }
        }
    }
}

fn clean_input(input: &str) -> Vec<String> {
    input
        .to_lowercase()
        .split_whitespace()
        .map(String::from)
        .collect()
}
