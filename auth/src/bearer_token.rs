use anyhow::Context;
use egg_mode::auth::{bearer_token, KeyPair, Token};

pub async fn generate_bearer_token() -> Result<Token, anyhow::Error> {
    // retreive api_key and api_key_secret from dotenv file
    let api_key = std::env::var("TWITTER_API_KEY").context("$TWITTER_API_KEY not set")?;
    let api_key_secret =
        std::env::var("TWITTER_API_KEY_SECRET").context("$TWITTER_API_KEY_SECRET not set")?;
    let key_pair = KeyPair::new(api_key, api_key_secret);
    let bearer_token = bearer_token(&key_pair)
        .await
        .map_err(|e| anyhow::Error::new(e));
    bearer_token
}
