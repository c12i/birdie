use std::env;

use twitter_v2::{authorization::Oauth1aToken,TwitterApi};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    // get keys and secrets from env vars
    let api_key = env::var("TWITTER_API_KEY").unwrap();
    let api_key_secret = env::var("TWITTER_API_KEY_SECRET").unwrap();
    let access_token = env::var("TWITTER_API_ACCESS_TOKEN").unwrap();
    let access_token_secret = env::var("TWITTER_API_ACCESS_TOKEN_SECRET").unwrap();

    // generate oauth1 access_token from keys and secrets
    let access_token =
        Oauth1aToken::new(api_key, api_key_secret, access_token, access_token_secret);

    // create twitter api client with our own access token
    let twitter_api_client = TwitterApi::new(access_token);

    // let's do something with the token i.e post a tweet
    let _tweet = twitter_api_client
        .post_tweet()
        .text("This is a random tweet".to_string())
        .send()
        .await
        .unwrap()
        .data()
        .unwrap();
    Ok(())
}
