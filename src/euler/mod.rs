use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::GraphClient;

// Create a simple module for scalar types
mod scalars;

// Define simple types for responses
pub type Decimal = String;

/// Represents an Euler vault market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    pub id: String,
    #[serde(rename = "totalShares")]
    pub total_shares: String,
    #[serde(rename = "totalBorrows")]
    pub total_borrows: String,
    #[serde(rename = "accumulatedFees")]
    pub accumulated_fees: String,
    pub cash: String,
    #[serde(rename = "interestAccumulator")]
    pub interest_accumulator: String,
    #[serde(rename = "interestRate")]
    pub interest_rate: String,
    pub timestamp: String,
}

/// Represents a token used in Euler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub decimals: i32,
}

/// Represents a deposit transaction in Euler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    pub id: String,
    pub sender: String,
    pub owner: String,
    pub assets: String,
    pub shares: String,
    pub vault: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

/// Represents a withdraw transaction in Euler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Withdraw {
    pub id: String,
    pub sender: String,
    pub receiver: String,
    pub owner: String,
    pub assets: String,
    pub shares: String,
    pub vault: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

/// Wrapper for Vault status responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultsResponse {
    #[serde(rename = "vaultStatuses")]
    pub vault_statuses: Vec<Vault>,
}

/// Wrapper for deposit transaction responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositsResponse {
    pub deposits: Vec<Deposit>,
}

/// Wrapper for withdraw transaction responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawsResponse {
    pub withdraws: Vec<Withdraw>,
}

/// Generic GraphQL response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

/// Fetch vault status information from the Euler subgraph, ordered by total shares
pub async fn fetch_vaults(client: &GraphClient, limit: i64) -> Result<VaultsResponse> {
    let query = r#"
    query EulerVaults($first: Int, $orderBy: VaultStatus_orderBy!, $orderDirection: OrderDirection!) {
        vaultStatuses(first: $first, orderBy: $orderBy, orderDirection: $orderDirection) {
            id
            totalShares
            totalBorrows
            accumulatedFees
            cash
            interestAccumulator
            interestRate
            timestamp
        }
    }
    "#;

    let variables = json!({
        "first": limit,
        "orderBy": "totalShares",
        "orderDirection": "desc"
    });

    let response: GraphQLResponse<VaultsResponse> = client.query_raw(query, variables).await?;
    Ok(response.data)
}

/// Fetch recent deposit transactions from the Euler subgraph
pub async fn fetch_deposits(client: &GraphClient, limit: i64) -> Result<DepositsResponse> {
    let query = r#"
    query EulerDeposits($first: Int, $orderBy: Deposit_orderBy!, $orderDirection: OrderDirection!) {
        deposits(first: $first, orderBy: $orderBy, orderDirection: $orderDirection) {
            id
            sender
            owner
            assets
            shares
            vault
            blockNumber
            blockTimestamp
            transactionHash
        }
    }
    "#;

    let variables = json!({
        "first": limit,
        "orderBy": "blockTimestamp",
        "orderDirection": "desc"
    });

    let response: GraphQLResponse<DepositsResponse> = client.query_raw(query, variables).await?;
    Ok(response.data)
}

/// Fetch recent withdraw transactions from the Euler subgraph
pub async fn fetch_withdraws(client: &GraphClient, limit: i64) -> Result<WithdrawsResponse> {
    let query = r#"
    query EulerWithdraws($first: Int, $orderBy: Withdraw_orderBy!, $orderDirection: OrderDirection!) {
        withdraws(first: $first, orderBy: $orderBy, orderDirection: $orderDirection) {
            id
            sender
            receiver
            owner
            assets
            shares
            vault
            blockNumber
            blockTimestamp
            transactionHash
        }
    }
    "#;

    let variables = json!({
        "first": limit,
        "orderBy": "blockTimestamp",
        "orderDirection": "desc"
    });

    let response: GraphQLResponse<WithdrawsResponse> = client.query_raw(query, variables).await?;
    Ok(response.data)
}
