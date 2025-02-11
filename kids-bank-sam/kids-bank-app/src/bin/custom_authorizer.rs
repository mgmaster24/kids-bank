use chrono::Utc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use kids_bank_sam::{get_token_secret, Claims};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct TokenAuthorizerEvent {
    #[serde(rename = "authorizationToken")]
    authorization_token: String,
    #[serde(rename = "methodArn")]
    method_arn: String,
}

#[derive(Serialize)]
struct AuthResponse {
    #[serde(rename = "principalId")]
    principal_id: String,
    #[serde(rename = "policyDocument")]
    policy_document: PolicyDocument,
}

#[derive(Serialize)]
struct PolicyDocument {
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Statement")]
    statement: Vec<Statement>,
}

#[derive(Serialize)]
struct Statement {
    #[serde(rename = "Action")]
    action: String,
    #[serde(rename = "Effect")]
    effect: String,
    #[serde(rename = "Resource")]
    resource: String,
}

async fn custom_authorizer(
    event: LambdaEvent<TokenAuthorizerEvent>,
) -> Result<AuthResponse, Error> {
    let token_opt = event.payload.authorization_token.strip_prefix("Bearer ");
    let auth_token = match token_opt {
        Some(t) => t,
        None => return Err("Missing authorization token".to_string().into()),
    };

    let token_secret_res = get_token_secret().await;
    if let Err(e) = token_secret_res {
        return Err(format!("Failed to get token secret. error: {}", e).into());
    }

    validate_token(auth_token, &token_secret_res.unwrap()).map(|claims| AuthResponse {
        principal_id: claims.sub().to_string(),
        policy_document: create_policy(&event.payload.method_arn),
    })
}

fn validate_token(token: &str, secret: &str) -> Result<Claims, Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    if Utc::now().timestamp() > token_data.claims.exp() as i64 {
        return Err("Token expired".into());
    }

    Ok(token_data.claims)
}

fn create_policy(method_arn: &str) -> PolicyDocument {
    PolicyDocument {
        version: "2012-10-17".to_string(),
        statement: vec![Statement {
            action: "execute-api:Invoke".to_string(),
            effect: "Allow".to_string(),
            resource: method_arn.to_string(),
        }],
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(custom_authorizer)).await
}
