mod lists_api;
mod spaces_api;
mod tweet_api;

use spaces_api::SpacesApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let api = SpacesApi::new().await?;

    let space = api.get_space("1zqKVXPQhvZJB".to_string()).await?;

    println!("{}", space.id);

    Ok(())
}
