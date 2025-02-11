// Use this code snippet in your app
// If you need more information about configurations or implementing the sample code, visit the AWS docs:
// https://docs.aws.amazon.com/sdk-for-rust/latest/dg/getting-started.html

use aws_config::{self, BehaviorVersion, Region};
use serde::Deserialize;

#[derive(Deserialize)]
struct TokenSecretJson {
    token_secret: String,
}

pub async fn get_token_secret() -> Result<String, String> {
    let secret_name = "kidsbank/token-secret";
    let region = Region::new("us-east-1");
    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region(region)
        .load()
        .await;

    let asm = aws_sdk_secretsmanager::Client::new(&config);
    let response = asm.get_secret_value().secret_id(secret_name).send().await;
    match response {
        Ok(res) => {
            let secret_json: Result<TokenSecretJson, serde_json::Error> =
                serde_json::from_str(res.secret_string().expect("Secret value should exist"));

            if let Err(e) = secret_json {
                return Err(format!("Error deserializing token secret. error: {}", e));
            }

            Ok(secret_json.unwrap().token_secret)
        }
        Err(e) => Err(format!("Could not get token secret {}", e)),
    }
}
