mod bookmarks_api;
mod lists_api;
mod spaces_api;
mod tweet_api;

#[allow(unused)]
use tweet_api::{
    delete_tweet, like_or_unlike_tweet, post_tweet, post_retweet,
    TweetLikeAction::{Like, UnLike},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    Ok(())
}
