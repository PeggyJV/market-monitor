# Market Monitor

A Rust crate for retrieving market data from DeFi lending platforms via The Graph API.

## Features

- Query market data from Morpho (Base chain) - TVL, interest rates, token info, etc.
- Easy API key configuration from environment variables
- Type-safe GraphQL queries using `graphql_client`
- Async/await support with Tokio

## Setup

1. Get an API key from [The Graph](https://thegraph.com/)
2. Set the API key in your environment:

```sh
export THE_GRAPH_API_KEY="your-api-key-here"
```

## Usage

```rust
use anyhow::Result;
use market_monitor::{init, morpho_base_client};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file (if present)
    init();

    // Create a client for the Morpho Base subgraph
    let client = morpho_base_client()?;

    // Fetch top 10 markets by TVL
    let markets = market_monitor::morpho::fetch_markets(&client, 10).await?;

    if let Some(market_data) = markets.first() {
        if let Some(markets) = &market_data.markets {
            for market in markets {
                println!("Market: {}", market.name);
                println!("- TVL: ${}", market.total_value_locked_usd);
                println!("- Token: {} ({})",
                        market.input_token.name,
                        market.input_token.symbol);
                // Access other market data...
            }
        }
    }

    // Fetch borrow rates
    let borrow_rates = market_monitor::morpho::fetch_borrow_rates(&client, 10).await?;

    // Fetch supply rates
    let supply_rates = market_monitor::morpho::fetch_supply_rates(&client, 10).await?;

    Ok(())
}
```

## Examples

See the `examples/` directory for more detailed examples:

- `morpho_markets.rs` - Retrieve and display market data from Morpho

Run examples with:

```
cargo run --example morpho_markets
```
