pub #[derive(Debug)]
struct Command {
    name: String,
    description: String,
    callback: fn(args &[&str]),
}

pub fn get_commands() Result<[]Command, String> {
    Err("Error".to_string())
}
