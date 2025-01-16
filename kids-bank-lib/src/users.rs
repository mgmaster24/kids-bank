use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
}

impl User {
    pub fn new(name: &str, email: &str, password: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(password);
        let hashed_pw = format!("{:X}", hasher.finalize());
        Self {
            name: name.to_string(),
            email: email.to_string(),
            password: hashed_pw,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn pw(&self) -> &String {
        &self.password
    }

    pub fn are_pws_equal(&self, pw: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(pw);
        let hpw = format!("{:X}", hasher.finalize());
        self.password == hpw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("Test User", "TestEmail@test.com", "my_password");
        assert_eq!(user.name, "Test User".to_string());
        assert_eq!(user.email, "TestEmail@test.com".to_string());
        assert!(user.are_pws_equal("my_password"));
    }
}
