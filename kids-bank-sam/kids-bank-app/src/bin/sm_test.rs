use kids_bank_sam::secrets_manager;

#[tokio::main]
async fn main() -> Result<(), String> {
    let token_secret_res = secrets_manager::get_token_secret().await;
    if let Err(e) = token_secret_res {
        return Err(format!("Failed to get token secret. error: {}", e));
    }

    println!("Token: {}", token_secret_res.unwrap());

    Ok(())
}
