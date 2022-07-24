mod lists_api;
mod tweet_api;

use lists_api::ListsApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let api = ListsApi::new().await?;
    api.delete_list(1550632065611300864).await?;
    Ok(())
}
