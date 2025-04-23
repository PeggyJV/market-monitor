use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::GraphClient;

// Create a simple module for scalar types
mod scalars;

// Define simple types for responses
pub type Decimal = String;

// Simple query struct for markets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub id: String,
    pub name: String,
    #[serde(rename = "inputToken")]
    pub input_token: Token,
    #[serde(rename = "borrowedToken")]
    pub borrowed_token: Token,
    #[serde(rename = "totalValueLockedUSD")]
    pub total_value_locked_usd: String,
    #[serde(rename = "totalDepositBalanceUSD")]
    pub total_deposit_balance_usd: String,
    #[serde(rename = "totalBorrowBalanceUSD")]
    pub total_borrow_balance_usd: String,
    #[serde(rename = "borrowingPositionCount")]
    pub borrowing_position_count: i32,
    #[serde(rename = "lendingPositionCount")]
    pub lending_position_count: i32,
    #[serde(rename = "openPositionCount")]
    pub open_position_count: i32,
    #[serde(rename = "maximumLTV")]
    pub maximum_ltv: String,
    #[serde(rename = "liquidationThreshold")]
    pub liquidation_threshold: String,
    #[serde(rename = "liquidationPenalty")]
    pub liquidation_penalty: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rate {
    pub id: String,
    pub rate: String,
    pub side: String,
    pub market: MarketRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketRef {
    pub id: String,
    pub name: String,
    #[serde(rename = "inputToken")]
    pub input_token: TokenRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRef {
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketsResponse {
    pub markets: Vec<Market>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatesResponse {
    #[serde(rename = "interestRates")]
    pub interest_rates: Vec<Rate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

/// Fetch markets from the Morpho subgraph, ordered by total value locked
pub async fn fetch_markets(client: &GraphClient, limit: i64) -> Result<MarketsResponse> {
    let query = r#"
    query MorphoMarkets($first: Int, $orderBy: Market_orderBy!, $orderDirection: OrderDirection!) {
        markets(first: $first, orderBy: $orderBy, orderDirection: $orderDirection) {
            id
            name
            inputToken {
                name
                symbol
                decimals
            }
            borrowedToken {
                name
                symbol
                decimals
            }
            totalValueLockedUSD
            totalDepositBalanceUSD
            totalBorrowBalanceUSD
            borrowingPositionCount
            lendingPositionCount
            openPositionCount
            maximumLTV
            liquidationThreshold
            liquidationPenalty
            isActive
            createdTimestamp
        }
    }
    "#;

    let variables = json!({
        "first": limit,
        "orderBy": "totalValueLockedUSD",
        "orderDirection": "desc"
    });

    let response: GraphQLResponse<MarketsResponse> = client.query_raw(query, variables).await?;
    Ok(response.data)
}

/// Fetch interest rates for the Morpho markets
pub async fn fetch_borrow_rates(client: &GraphClient, limit: i64) -> Result<RatesResponse> {
    let query = r#"
    query MorphoBorrowRates($first: Int, $side: InterestRateSide!) {
        interestRates(first: $first, where: { side: $side }) {
            id
            rate
            side
            market {
                id
                name
                inputToken {
                    symbol
                }
            }
        }
    }
    "#;

    let variables = json!({
        "first": limit,
        "side": "BORROWER",
    });

    let response: GraphQLResponse<RatesResponse> = client.query_raw(query, variables).await?;
    Ok(response.data)
}

/// Fetch supply rates for the Morpho markets
pub async fn fetch_supply_rates(client: &GraphClient, limit: i64) -> Result<RatesResponse> {
    let query = r#"
    query MorphoSupplyRates($first: Int, $side: InterestRateSide!) {
        interestRates(first: $first, where: { side: $side }) {
            id
            rate
            side
            market {
                id
                name
                inputToken {
                    symbol
                }
            }
        }
    }
    "#;

    let variables = json!({
        "first": limit,
        "side": "LENDER",
    });

    let response: GraphQLResponse<RatesResponse> = client.query_raw(query, variables).await?;
    Ok(response.data)
}
