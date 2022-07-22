use anyhow::Context;
use egg_mode::{tweet::DraftTweet, KeyPair};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    // retreive api_key and api_key_secret from dotenv file
    let api_key = std::env::var("TWITTER_API_KEY").context("$TWITTER_API_KEY not set")?;
    let api_key_secret =
        std::env::var("TWITTER_API_KEY_SECRET").context("$TWITTER_API_KEY_SECRET not set")?;

    let key_pair = KeyPair::new(api_key, api_key_secret);

    // "oob" is needed for PIN-based auth
    let request_token = egg_mode::auth::request_token(&key_pair, "oob")
        .await
        .map_err(|e| anyhow::Error::new(e))?;
    let auth_url = egg_mode::auth::authenticate_url(&request_token);

    println!("Go to the following URL, sign in, and give me the PIN that comes back:\n");
    println!("{auth_url}");

    // accept the verifier pin as a command line prompt
    let mut pin = String::new();
    println!("please input the 7 digit pin:");
    std::io::stdin()
        .read_line(&mut pin)
        .context("Error: Could not read a line")?;

    // get access token from twitter
    let (token, user_id, screen_name) = egg_mode::auth::access_token(key_pair, &request_token, pin)
        .await
        .map_err(|e| anyhow::Error::new(e))?;

    // print output on successfult login
    println!("{user_id} {screen_name}");

    // use our token to post a tweet
    let post = DraftTweet::new("spam")
        .send(&token)
        .await
        .map_err(|e| anyhow::Error::new(e))?;
    println!("{}", post.response.text);
    Ok(())
}
