# Market Monitor

A Rust crate for retrieving market data from DeFi lending platforms via The Graph API.

## Features

- Query market data from Morpho (Base chain) - TVL, interest rates, token info, etc.
- Query vault data from Euler - vault status, deposits, withdrawals, etc.
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
use market_monitor::{init, morpho_base_client, euler_client};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file (if present)
    init();

    // Create a client for the Morpho Base subgraph
    let morpho_client = morpho_base_client()?;

    // Fetch top 10 markets by TVL from Morpho
    let markets = market_monitor::morpho::fetch_markets(&morpho_client, 10).await?;

    if let Some(market_data) = markets.markets.first() {
        println!("Market: {}", market_data.name);
        println!("- TVL: ${}", market_data.total_value_locked_usd);
        println!("- Token: {} ({})",
                market_data.input_token.name,
                market_data.input_token.symbol);
    }

    // Create a client for the Euler subgraph
    let euler_client = euler_client()?;

    // Fetch top 10 vaults by total shares from Euler
    let vaults = market_monitor::euler::fetch_vaults(&euler_client, 10).await?;

    if let Some(vault) = vaults.vault_statuses.first() {
        println!("Vault: {}", vault.id);
        println!("- Total Shares: {}", vault.total_shares);
        println!("- Total Borrows: {}", vault.total_borrows);
        println!("- Interest Rate: {}", vault.interest_rate);
    }

    // Fetch recent deposits and withdrawals
    let deposits = market_monitor::euler::fetch_deposits(&euler_client, 10).await?;
    let withdraws = market_monitor::euler::fetch_withdraws(&euler_client, 10).await?;

    Ok(())
}
```

## Examples

See the `examples/` directory for more detailed examples:

- `morpho_markets.rs` - Retrieve and display market data from Morpho
- `euler_vaults.rs` - Retrieve and display vault data from Euler

Run examples with:

```
cargo run --example morpho_markets
cargo run --example euler_vaults
```
