use anyhow::{Context, Ok};
use birdie_auth::generate_pregenerated_access_token;
use egg_mode::tweet::DraftTweet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    // generate access token - sign in as yourself on twitter
    let token = generate_pregenerated_access_token().await?;
    // use the token to post a random tweet
    let post = DraftTweet::new("spam again")
        .send(&token)
        .await
        .map_err(|e| anyhow::Error::new(e))?;
    println!("{}", post.response.text);
    Ok(())
}
