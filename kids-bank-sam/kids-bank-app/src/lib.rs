use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAccountRequestBody {
    pub name: String,
    pub email: String,
    pub password: String,
}
