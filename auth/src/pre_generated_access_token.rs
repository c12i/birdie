use anyhow::Context;
use egg_mode::Token;

pub async fn generate_pregenerated_access_token() -> Result<Token, anyhow::Error> {
    // retreive api_key, access_token, api_key_secret and access_token_secret from dotenv file
    let api_key = std::env::var("TWITTER_API_KEY").context("$TWITTER_API_KEY not set")?;
    let api_key_secret =
        std::env::var("TWITTER_API_KEY_SECRET").context("$TWITTER_API_KEY_SECRET not set")?;
    dotenv::dotenv().ok();
    let access_token = std::env::var("TWITTER_API_ACCESS_TOKEN").context("$TWITTER_API_ACCESS_TOKEN not set")?;
    let access_token_secret =
        std::env::var("TWITTER_API_ACCESS_TOKEN_SECRET").context("$TWITTER_API_ACCESS_TOKEN_SECRET not set")?;
    // create keypairs
    let con_token = egg_mode::KeyPair::new(api_key, api_key_secret);
    let access_token = egg_mode::KeyPair::new(access_token, access_token_secret);
    // create pre-generated access token to sigin in to twitter as yourself
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    Ok(token)
}
