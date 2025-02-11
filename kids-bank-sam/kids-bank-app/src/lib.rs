mod secretsmanager;

pub use secretsmanager::get_token_secret;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String, exp: usize) -> Self {
        Claims { sub, exp }
    }

    pub fn sub(&self) -> &str {
        &self.sub
    }

    pub fn exp(&self) -> usize {
        self.exp
    }
}
