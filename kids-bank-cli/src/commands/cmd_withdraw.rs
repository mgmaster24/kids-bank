use crate::db::Client;
use kids_bank_lib::AccountHandler;

pub fn withdraw(client: &Client, args: &[String]) {
    if args.is_empty() || args.len() > 2 {
        println!("No arguments necessary for this method");
        return;
    }

    let id = args[0].as_str();
    let balance = args[1].parse::<f64>().unwrap();
    let withdraw_res = client.withdraw(id, balance);
    match withdraw_res {
        Ok(new_balance) => {
            println!("Successfully withdrew ${} from account.", balance);
            println!("New Balance: {}", new_balance);
        }
        Err(e) => println!("{}", e),
    }
}
