pub mod secrets_manager;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
