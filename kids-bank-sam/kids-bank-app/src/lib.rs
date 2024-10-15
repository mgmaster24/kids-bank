pub mod dynamo_db;

use serde::{Deserialize, Serialize};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
pub struct AccountRequest {
    pub id: Option<u64>,
    pub email: Option<String>,
    pub amount: Option<f64>,
}

#[derive(Deserialize)]
pub struct CreateAcctRequest {
    pub email: String,
    pub name: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses   
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
pub struct AccountResponse {
    pub status_code: i32,
    pub body: String,
}

// Do we need this?
//impl AccountRequest {
//
//}

impl AccountResponse {
    pub fn success(body: &str) -> Self {
        AccountResponse {
            status_code: 200,
            body: body.to_string(),
        }
    }

    pub fn fail(err: &str) -> Self {
        AccountResponse {
            status_code: 500,
            body: err.to_string(),
        }
    }
}
