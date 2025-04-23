use anyhow::Result;
use chrono;
use log::{debug, error, info};
use market_monitor;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger with debug level
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // Load environment variables
    info!("Initializing environment");
    market_monitor::init();

    // Create client for Morpho Base subgraph
    info!("Creating Morpho API client");
    let client = market_monitor::morpho_base_client()?;

    // Fetch markets from Morpho
    info!("Fetching Morpho markets...");
    match market_monitor::morpho::fetch_markets(&client, 10).await {
        Ok(markets) => {
            info!("Successfully fetched {} markets", markets.markets.len());

            for market in &markets.markets {
                info!("Market: {}", market.name);
                info!("  - TVL: ${}", market.total_value_locked_usd);
                info!(
                    "  - Input Token: {} ({})",
                    market.input_token.name, market.input_token.symbol
                );
                info!(
                    "  - Borrowed Token: {} ({})",
                    market.borrowed_token.name, market.borrowed_token.symbol
                );
                debug!("  - Market ID: {}", market.id);
                info!("  - LTV: {}%", market.maximum_ltv);
                info!(
                    "  - Liquidation Threshold: {}%",
                    market.liquidation_threshold
                );
                info!("  - Liquidation Penalty: {}%", market.liquidation_penalty);
                info!("  - Active: {}", market.is_active);
                info!("  - Open Positions: {}", market.open_position_count);
                info!(
                    "  - Created: {}",
                    chrono::DateTime::from_timestamp(
                        market.created_timestamp.parse::<i64>().unwrap(),
                        0
                    )
                    .unwrap()
                );
                info!("---");
            }

            // Fetch borrow rates
            info!("Fetching Morpho borrow rates...");
            match market_monitor::morpho::fetch_borrow_rates(&client, 10).await {
                Ok(borrow_rates) => {
                    info!(
                        "Successfully fetched {} borrow rates",
                        borrow_rates.interest_rates.len()
                    );

                    for rate in &borrow_rates.interest_rates {
                        info!("Borrow Rate:");
                        info!("  - Rate: {}%", calculate_apy(&rate.rate));
                        info!("  - Market: {}", rate.market.name);
                        info!("  - Token: {}", rate.market.input_token.symbol);
                        debug!("  - Rate ID: {}", rate.id);
                        info!("---");
                    }
                }
                Err(e) => {
                    error!("Failed to fetch borrow rates: {}", e);
                }
            }

            // Fetch supply rates
            info!("Fetching Morpho supply rates...");
            match market_monitor::morpho::fetch_supply_rates(&client, 10).await {
                Ok(supply_rates) => {
                    info!(
                        "Successfully fetched {} supply rates",
                        supply_rates.interest_rates.len()
                    );

                    for rate in &supply_rates.interest_rates {
                        info!("Supply Rate:");
                        info!("  - Rate: {}%", calculate_apy(&rate.rate));
                        info!("  - Market: {}", rate.market.name);
                        info!("  - Token: {}", rate.market.input_token.symbol);
                        debug!("  - Rate ID: {}", rate.id);
                        info!("---");
                    }
                }
                Err(e) => {
                    error!("Failed to fetch supply rates: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch markets: {}", e);
        }
    }

    Ok(())
}

/// Calculate APY from rate (e^rate - 1)
fn calculate_apy(rate: &str) -> f64 {
    match rate.parse::<f64>() {
        Ok(r) => (r.exp() - 1.0) * 100.0,
        Err(e) => {
            error!("Failed to parse rate '{}': {}", rate, e);
            0.0
        }
    }
}
