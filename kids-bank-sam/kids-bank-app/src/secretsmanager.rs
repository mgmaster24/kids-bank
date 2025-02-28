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
    let response = asm
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await
        .map_err(|e| format!("Could not get token secret {}", e))?;
    let secret_string = response
        .secret_string()
        .ok_or_else(|| "Secret value is missing".to_string())?;
    let secret_json: TokenSecretJson = serde_json::from_str(secret_string)
        .map_err(|e| format!("Error deserializing token secret: {}", e))?;
    Ok(secret_json.token_secret)
}
