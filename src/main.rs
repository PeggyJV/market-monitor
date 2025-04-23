use anyhow::Result;
use log::info;
use market_monitor::{init, morpho_base_client};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger and environment
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    init();

    info!("Initializing Morpho client...");
    let client = morpho_base_client()?;

    info!("Fetching top markets by TVL...");
    let markets = market_monitor::morpho::fetch_markets(&client, 5).await?;

    for market in &markets.markets {
        info!(
            "Market: {} - TVL: ${}",
            market.name, market.total_value_locked_usd
        );
    }

    Ok(())
}
