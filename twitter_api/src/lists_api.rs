use birdie_auth::generate_oath1_token;
use twitter_v2::{authorization::Oauth1aToken, data::List, TwitterApi, User};

pub struct ListsApi {
    twitter_client: TwitterApi<Oauth1aToken>,
}

impl ListsApi {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let access_token = generate_oath1_token().await?;
        let twitter_client = TwitterApi::new(access_token);
        Ok(ListsApi { twitter_client })
    }

    pub async fn post_public_list(
        &self,
        name: String,
        description: String,
    ) -> Result<List, anyhow::Error> {
        let list = self
            .twitter_client
            .post_list(name)
            .description(description)
            .private(false)
            .send()
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .into_data()
            .ok_or_else(|| anyhow::Error::msg("Failed to retreive the list"))?;
        Ok(list)
    }

    pub async fn update_list(
        &self,
        list_id: u64,
        name: String,
        description: String,
    ) -> Result<bool, anyhow::Error> {
        let updated_list = self
            .twitter_client
            .put_list(list_id)
            .name(name)
            .description(description)
            .send()
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .into_data()
            .ok_or_else(|| anyhow::Error::msg("Failed to get data"))?;
        Ok(updated_list.as_bool())
    }

    pub async fn add_member(&self, list_id: u64, user_id: u64) -> Result<bool, anyhow::Error> {
        let is_member = self
            .twitter_client
            .post_list_member(list_id, user_id)
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .into_data()
            .ok_or_else(|| anyhow::Error::msg("Failed to get data"))?;
        Ok(is_member.as_bool())
    }

    pub async fn follow_list(&self, list_id: u64) -> Result<bool, anyhow::Error> {
        let following = self
            .twitter_client
            .with_user_ctx()
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .post_my_followed_list(list_id)
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .into_data()
            .ok_or_else(|| anyhow::Error::msg("Failed to get data"))?;
        Ok(following.as_bool())
    }

    pub async fn pin_list(&self, list_id: u64) -> Result<bool, anyhow::Error> {
        let pinned = self
            .twitter_client
            .with_user_ctx()
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .post_my_pinned_list(list_id)
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .into_data()
            .ok_or_else(|| anyhow::Error::msg("Failed to get data"))?;
        Ok(pinned.as_bool())
    }

    pub async fn unpin_list(&self, list_id: u64) -> Result<bool, anyhow::Error> {
        let pinned = self
            .twitter_client
            .with_user_ctx()
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .delete_my_pinned_list(list_id)
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .into_data()
            .ok_or_else(|| anyhow::Error::msg("Failed to get data"))?;
        Ok(pinned.as_bool())
    }

    pub async fn delete_list(&self, list_id: u64) -> Result<bool, anyhow::Error> {
        let deleted = self
            .twitter_client
            .delete_list(list_id)
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .into_data()
            .ok_or_else(|| anyhow::Error::msg("Failed to get data"))?;
        Ok(deleted.as_bool())
    }
}
