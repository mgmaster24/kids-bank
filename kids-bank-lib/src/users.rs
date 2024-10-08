pub struct User {
    name: String,
    email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
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
        let user = User::new("Test User".to_string(), "TestEmail@test.com".to_string());
        assert_eq!(user.name, "Test User".to_string());
        assert_eq!(user.email, "TestEmail@test.com".to_string());
    }
}
