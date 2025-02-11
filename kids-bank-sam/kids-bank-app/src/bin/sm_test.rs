use kids_bank_sam::get_token_secret;

#[tokio::main]
async fn main() -> Result<(), String> {
    let token_secret_res = get_token_secret().await;
    if let Err(e) = token_secret_res {
        return Err(format!("Failed to get token secret. error: {}", e));
    }

    println!("Token: {}", token_secret_res.unwrap());

    Ok(())
}
