mod bookmarks_api;
mod lists_api;
mod spaces_api;
mod tweet_api;

use lists_api::ListsApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let api = ListsApi::new().await?;
    api.post_public_list("test list".to_string(), "testing".to_string())
        .await?;
    Ok(())
}
