use std::env;

use anyhow::Context;
use twitter_v2::{authorization::BearerToken, query::TweetField, TwitterApi};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let bearer_token =
        env::var("TWITTER_API_BEARER_TOKEN").context("$TWITTER_API_BEARER_TOKEN is not set")?;
    let bearer_token = BearerToken::new(bearer_token);
    // create twitter client
    let twitter_client = TwitterApi::new(bearer_token);
    // get a tweet
    let tweet = twitter_client
        .get_tweet(20)
        .tweet_fields([
            TweetField::AuthorId,
            TweetField::Text,
        ])
        .send()
        .await
        .map_err(|e| anyhow::Error::new(e))?
        .into_data()
        .ok_or_else(|| anyhow::Error::msg("No tweet data found"))?;
    let tweet_text= tweet.text;
    let tweet_author_id = tweet.author_id.unwrap();
    println!("Author id: {tweet_author_id}");
    println!("Tweet: {tweet_text}");
    Ok(())
}
