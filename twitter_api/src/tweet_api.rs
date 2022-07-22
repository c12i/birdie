use birdie_auth::generate_oath1_token;
use twitter_v2::{authorization::Oauth1aToken, Tweet, TwitterApi, User};

pub struct TweetApi {
    twitter_client: TwitterApi<Oauth1aToken>,
}

pub enum TweetLikeAction {
    Like,
    UnLike,
}

impl TweetApi {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let access_token = generate_oath1_token().await?;
        let twitter_client = TwitterApi::new(access_token);
        Ok(TweetApi { twitter_client })
    }

    pub async fn get_me(&self) -> Result<User, anyhow::Error> {
        let me = self
            .twitter_client
            .get_users_me()
            .send()
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .into_data()
            .ok_or_else(|| anyhow::Error::msg("Failed to get user data"))?;
        Ok(me)
    }

    pub async fn post_tweet(
        &self,
        text: String,
        in_reply_to_tweet_id: Option<u64>,
    ) -> Result<Tweet, anyhow::Error> {
        let mut tweet = self.twitter_client.post_tweet();
        if let Some(tweet_id) = in_reply_to_tweet_id {
            tweet.in_reply_to_tweet_id(tweet_id);
        }
        let tweet = tweet
            .text(text)
            .send()
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .into_data()
            .ok_or_else(|| anyhow::Error::msg("Failed to get tweet data"))?;
        Ok(tweet)
    }

    pub async fn post_retweet(&self, tweet_id: u64) -> Result<(), anyhow::Error> {
        if let Ok(me) = self.get_me().await {
            self.twitter_client
                .post_user_retweet(me.id, tweet_id)
                .await
                .map_err(|e| anyhow::Error::new(e))?;
        }
        Ok(())
    }

    pub async fn delete_tweet(&self, tweet_id: u64) -> Result<(), anyhow::Error> {
        self.twitter_client
            .delete_tweet(tweet_id)
            .await
            .map_err(|e| anyhow::Error::new(e))?;
        Ok(())
    }

    pub async fn like_or_unlike_tweet(
        &self,
        tweet_id: u64,
        action: TweetLikeAction,
    ) -> Result<(), anyhow::Error> {
        if let Ok(me) = self.get_me().await {
            match action {
                TweetLikeAction::Like => {
                    self.twitter_client
                        .post_user_like(me.id, tweet_id)
                        .await
                        .map_err(|e| anyhow::Error::new(e))?;
                }
                TweetLikeAction::UnLike => {
                    self.twitter_client
                        .delete_user_like(me.id, tweet_id)
                        .await
                        .map_err(|e| anyhow::Error::new(e))?;
                }
            }
        }
        Ok(())
    }
}
