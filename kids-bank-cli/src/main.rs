mod commands;
mod db;
mod repl;

fn main() {
    match db::sqlite_db::Client::init("./accounts.db") {
        Ok(c) => {
            repl::start_repl(&c);
        }
        Err(e) => println!("Couldn't create sql client. Reason: {}", e),
    }
}
