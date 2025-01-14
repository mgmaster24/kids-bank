use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i64,
    name: String,
    email: String,
    password: String,
}

impl User {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            email: email.to_string(),
            password: "".to_string(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn email(&self) -> &String {
        &self.email
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("Test User", "TestEmail@test.com");
        assert_eq!(user.name, "Test User".to_string());
        assert_eq!(user.email, "TestEmail@test.com".to_string());
    }
}
