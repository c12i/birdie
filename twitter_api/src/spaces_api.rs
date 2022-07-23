use birdie_auth::generate_oath1_token;
use twitter_v2::{authorization::Oauth1aToken, Space, TwitterApi};

pub struct SpacesApi {
    twitter_client: TwitterApi<Oauth1aToken>,
}

impl SpacesApi {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let access_token = generate_oath1_token().await?;
        let twitter_client = TwitterApi::new(access_token);
        Ok(SpacesApi { twitter_client })
    }

    pub async fn get_space(&self, space_id: String) -> Result<Space, anyhow::Error> {
        let space = self
            .twitter_client
            .get_space(space_id)
            .send()
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .into_data()
            .ok_or_else(|| anyhow::Error::msg("Failed to get data"))?;
        Ok(space)
    }
}
