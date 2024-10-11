mod acct_management;
mod users;

pub fn create_user_account(name: &str, email: &str) -> Result<acct_management::Account, String> {
    let user = users::User::new(name.to_string(), email.to_string());
    Ok(acct_management::Account::new(user))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        create_user_account("some user", "someuser@email.com");
    }
}
