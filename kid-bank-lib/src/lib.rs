mod acct_management;
mod dynamo_db;
mod users;

pub fn create_user(name: &str, email: &str) {
    users::User::new(name.to_string(), email.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
