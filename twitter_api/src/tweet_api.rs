use birdie_auth::generate_oath1_token;
use twitter_v2::{id::IntoNumericId, Tweet, TwitterApi};

pub async fn post_tweet(
    text: String,
    in_reply_to_tweet_id: Option<impl IntoNumericId>,
) -> Result<Tweet, anyhow::Error> {
    let access_token = generate_oath1_token().await?;
    let twitter_client = TwitterApi::new(access_token);
    let mut tweet = twitter_client.post_tweet();
    if let Some(tweet_id) = in_reply_to_tweet_id {
        tweet.in_reply_to_tweet_id(tweet_id);
    }
    let tweet = tweet
        .send()
        .await
        .map_err(|e| anyhow::Error::new(e))?
        .into_data()
        .ok_or_else(|| anyhow::Error::msg("Failed to get tweet data"))?;
    Ok(tweet)
}

pub async fn delete_tweet(tweet_id: impl IntoNumericId) -> Result<(), anyhow::Error> {
    let access_token = generate_oath1_token().await?;
    let twitter_client = TwitterApi::new(access_token);
    twitter_client
        .delete_tweet(tweet_id)
        .await
        .map_err(|e| anyhow::Error::new(e))?;
    Ok(())
}

pub enum TweetLikeAction {
    Like,
    UnLike,
}

pub async fn like_or_unlike_tweet(
    tweet_id: impl IntoNumericId,
    action: TweetLikeAction,
) -> Result<(), anyhow::Error> {
    let access_token = generate_oath1_token().await?;
    let twitter_client = TwitterApi::new(access_token);
    if let Some(me) = twitter_client
        .get_users_me()
        .send()
        .await
        .map_err(|e| anyhow::Error::new(e))?
        .into_data()
    {
        match action {
            TweetLikeAction::Like => {
                twitter_client
                    .post_user_like(me.id, tweet_id)
                    .await
                    .map_err(|e| anyhow::Error::new(e))?;
            }
            TweetLikeAction::UnLike => {
                twitter_client
                    .delete_user_like(me.id, tweet_id)
                    .await
                    .map_err(|e| anyhow::Error::new(e))?;
            }
        }
    }
    Ok(())
}
