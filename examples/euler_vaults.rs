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

    // Create client for Euler subgraph
    info!("Creating Euler API client");
    let client = market_monitor::euler_client()?;

    // Fetch vaults from Euler
    info!("Fetching Euler vaults...");
    match market_monitor::euler::fetch_vaults(&client, 10).await {
        Ok(vaults) => {
            info!(
                "Successfully fetched {} vaults",
                vaults.vault_statuses.len()
            );

            for vault in &vaults.vault_statuses {
                info!("Vault: {}", vault.id);
                info!("  - Total Shares: {}", vault.total_shares);
                info!("  - Total Borrows: {}", vault.total_borrows);
                info!("  - Cash: {}", vault.cash);
                info!("  - Interest Rate: {}", calculate_apy(&vault.interest_rate));
                info!("  - Accumulated Fees: {}", vault.accumulated_fees);
                info!(
                    "  - Last Update: {}",
                    chrono::DateTime::from_timestamp(
                        vault.timestamp.parse::<i64>().unwrap_or(0),
                        0
                    )
                    .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap())
                );
                info!("---");
            }

            // Fetch recent deposits
            info!("Fetching Euler deposits...");
            match market_monitor::euler::fetch_deposits(&client, 5).await {
                Ok(deposits) => {
                    info!("Successfully fetched {} deposits", deposits.deposits.len());

                    for deposit in &deposits.deposits {
                        info!("Deposit:");
                        info!("  - Vault: {}", deposit.vault);
                        info!("  - Amount: {} assets", deposit.assets);
                        info!("  - Shares: {}", deposit.shares);
                        info!("  - Sender: {}", deposit.sender);
                        info!("  - Owner: {}", deposit.owner);
                        info!(
                            "  - Time: {}",
                            chrono::DateTime::from_timestamp(
                                deposit.block_timestamp.parse::<i64>().unwrap_or(0),
                                0
                            )
                            .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap())
                        );
                        debug!("  - TX Hash: {}", deposit.transaction_hash);
                        info!("---");
                    }
                }
                Err(e) => {
                    error!("Failed to fetch deposits: {}", e);
                }
            }

            // Fetch recent withdrawals
            info!("Fetching Euler withdrawals...");
            match market_monitor::euler::fetch_withdraws(&client, 5).await {
                Ok(withdraws) => {
                    info!(
                        "Successfully fetched {} withdrawals",
                        withdraws.withdraws.len()
                    );

                    for withdraw in &withdraws.withdraws {
                        info!("Withdrawal:");
                        info!("  - Vault: {}", withdraw.vault);
                        info!("  - Amount: {} assets", withdraw.assets);
                        info!("  - Shares: {}", withdraw.shares);
                        info!("  - Sender: {}", withdraw.sender);
                        info!("  - Receiver: {}", withdraw.receiver);
                        info!("  - Owner: {}", withdraw.owner);
                        info!(
                            "  - Time: {}",
                            chrono::DateTime::from_timestamp(
                                withdraw.block_timestamp.parse::<i64>().unwrap_or(0),
                                0
                            )
                            .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap())
                        );
                        debug!("  - TX Hash: {}", withdraw.transaction_hash);
                        info!("---");
                    }
                }
                Err(e) => {
                    error!("Failed to fetch withdrawals: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch vaults: {}", e);
        }
    }

    Ok(())
}

/// Calculate APY from rate (e^rate - 1)
fn calculate_apy(rate: &str) -> String {
    match rate.parse::<f64>() {
        Ok(r) => format!("{}%", (r.exp() - 1.0) * 100.0),
        Err(e) => {
            error!("Failed to parse rate '{}': {}", rate, e);
            "N/A".to_string()
        }
    }
}
