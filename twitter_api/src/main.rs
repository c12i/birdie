mod lists_api;
mod tweet_api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    Ok(())
}
