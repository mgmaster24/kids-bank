use kids_bank_sam::get_token_secret;

#[tokio::main]
async fn main() -> Result<(), String> {
    let token_secret = get_token_secret()
        .await
        .map_err(|e| format!("Failed to get token secret. error: {}", e));

    println!("Token: {:?}", token_secret);
    Ok(())
}
