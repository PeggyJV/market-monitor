//! # Market Monitor
//!
//! A Rust crate for retrieving market data from DeFi lending platforms via The Graph.
//! This crate supports querying Morpho and other lending protocols (to be added).

mod client;
mod config;
pub mod euler;
pub mod morpho;

use anyhow::Result;
use url::Url;

// Re-export essential types
pub use client::GraphClient;

/// Initializes the environment by loading variables from .env file
pub fn init() {
    dotenv::dotenv().ok();
    // API key is required for The Graph's gateway
}

/// Creates a new client for querying the Morpho Base subgraph
pub fn morpho_base_client() -> Result<GraphClient> {
    let subgraph_id = config::morpho_base_subgraph_id();
    let url = Url::parse(&config::subgraph_url(subgraph_id))
        .map_err(|e| anyhow::anyhow!("Invalid subgraph URL: {}", e))?;

    GraphClient::new(url)
}

/// Creates a new client for querying the Euler protocol subgraph
pub fn euler_client() -> Result<GraphClient> {
    let subgraph_id = config::euler_subgraph_id();
    let url = Url::parse(&config::subgraph_url(subgraph_id))
        .map_err(|e| anyhow::anyhow!("Invalid subgraph URL: {}", e))?;

    GraphClient::new(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_morpho_markets_query() {
        // Skip this test if no API key is set
        if std::env::var("THE_GRAPH_API_KEY").is_err() {
            println!("Skipping test_morpho_markets_query: No API key set");
            return;
        }

        init();
        let client = morpho_base_client().expect("Failed to create client");
        let markets = morpho::fetch_markets(&client, 10)
            .await
            .expect("Failed to fetch markets");

        assert!(!markets.markets.is_empty());
    }
}
