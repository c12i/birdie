mod bookmarks_api;
mod lists_api;
mod spaces_api;
mod tweet_api;

#[allow(unused)]
use tweet_api::{
    TweetApi,
    TweetLikeAction::{Like, UnLike},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let api = TweetApi::new().await?;
    let _tweet = api.post_tweet("@collinsmuriuki".to_string(), None).await?;
    Ok(())
}
