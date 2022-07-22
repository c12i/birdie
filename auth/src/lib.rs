use std::env;

use anyhow::Context;
use twitter_v2::authorization::Oauth1aToken;

// generate oauth1 access token to access the twitter api on behalf of yourself
pub async fn generate_oath1_token() -> Result<Oauth1aToken, anyhow::Error> {
    // get keys and secrets from env vars
    let api_key = env::var("TWITTER_API_KEY").context("$TWITTER_API_KEY is not set")?;
    let api_key_secret =
        env::var("TWITTER_API_KEY_SECRET").context("$TWITTER_API_KEY_SECRET is not set")?;
    let access_token =
        env::var("TWITTER_API_ACCESS_TOKEN").context("$TWITTER_API_ACCESS_TOKEN is not set")?;
    let access_token_secret = env::var("TWITTER_API_ACCESS_TOKEN_SECRET")
        .context("$TWITTER_API_ACCESS_TOKEN_SECRET is not set")?;
    // generate oauth1 access_token from keys and secrets
    let access_token =
        Oauth1aToken::new(api_key, api_key_secret, access_token, access_token_secret);
    Ok(access_token)
}
