mod commands;
mod db;
mod repl;

fn main() {
    match db::sqlite_db::Client::open("./accounts.db") {
        Ok(conn) => {
            let tbl_res = conn.create_table();
            if tbl_res.is_err() {
                println!(
                    "Error creating the accounts table: {}",
                    tbl_res.expect_err("Should be error")
                );
                return;
            }

            repl::start_repl(&conn);
        }
        Err(e) => println!("Couldn't create sql client. Reason: {}", e),
    }
}
