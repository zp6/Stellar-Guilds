/**
 * Decentralized Oracle Price Feed Trait. Fixes GalactiGuild/Stellar-Guilds#237.
 * Standard trait definitions for consuming external Oracle prices.
 */
use soroban_sdk::{contracttype, Address, BytesN, Env, Vec};

#[contracttype]
pub trait PriceOracleTrait {
    /// Get the latest price for an asset.
    /// Returns the price in i128 (scaled by 1e7 for precision).
    fn get_latest_price(env: &Env, asset_id: BytesN<32>) -> i128;
    
    /// Get the timestamp of the last price update.
    fn get_last_update_time(env: &Env, asset_id: BytesN<32>) -> u64;
}

/// Dummy oracle for testing - returns hardcoded price sequence.
#[contracttype]
pub struct DummyOracle {
    pub prices: Vec<i128>,
    pub index: u32,
}

impl PriceOracleTrait for DummyOracle {
    fn get_latest_price(env: &Env, asset_id: BytesN<32>) -> i128 {
        // Return a fixed test price (e.g., $1.00 = 10_000_000 scaled)
        10_000_000
    }
    
    fn get_last_update_time(env: &Env, asset_id: BytesN<32>) -> u64 {
        env.ledger().timestamp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_oracle_price() {
        let env = Env::default();
        let asset_id = BytesN::from_array(&env, &[1u8; 32]);
        let price = DummyOracle::get_latest_price(&env, asset_id);
        assert_eq!(price, 10_000_000);
    }

    #[test]
    fn test_dummy_oracle_timestamp() {
        let env = Env::default();
        env.ledger().set_timestamp(1000);
        let asset_id = BytesN::from_array(&env, &[1u8; 32]);
        let ts = DummyOracle::get_last_update_time(&env, asset_id);
        assert_eq!(ts, 1000);
    }
}
