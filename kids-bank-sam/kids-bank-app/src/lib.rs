mod secretsmanager;

use http::StatusCode;
use lambda_http::Error;
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

pub fn response_error(status: u16, msg: &str) -> Error {
    let status_code = StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    Error::from(format!("{}: {}", status_code, msg))
}
