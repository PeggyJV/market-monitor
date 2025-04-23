use log::{debug, info};
use std::env;

/// The Graph API key.
/// Expects `THE_GRAPH_API_KEY` to be set in the environment.
pub fn graph_api_key() -> String {
    let key = env::var("THE_GRAPH_API_KEY")
        .expect("`THE_GRAPH_API_KEY` not found — set it in your .env or shell");

    debug!(
        "Using Graph API key: {}",
        key.chars().take(4).collect::<String>() + "****"
    );
    key
}

/// Build a full subgraph URL for a given subgraph ID.
/// We'll try different URL formats since The Graph API structure might have changed
pub fn subgraph_url(id: &str) -> String {
    // Use the API endpoint format for The Graph's gateway
    let url = format!(
        "https://gateway.thegraph.com/api/{}/subgraphs/id/{}",
        graph_api_key(),
        id
    );

    info!("Using The Graph gateway API endpoint");
    debug!("Full subgraph URL: {}", url);
    url
}

/// Returns the Morpho Base subgraph ID
pub fn morpho_base_subgraph_id() -> &'static str {
    // Using Morpho's Base subgraph ID
    let id = "71ZTy1veF9twER9CLMnPWeLQ7GZcwKsjmygejrgKirqs";
    debug!("Using Morpho Base subgraph ID: {}", id);
    id
}
