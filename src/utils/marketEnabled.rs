#[path = "../config/chainConfig.rs"]
mod chain_config;
use chain_config::MOVEMENT_DEVNET_CHAIN_ID;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use alloy::primitives::Address;

pub static ENABLED_MARKETS: Lazy<HashMap<u32, Vec<Address>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        MOVEMENT_DEVNET_CHAIN_ID,
        vec![
            "0xD27B8001E00da3018c604d98c1d3715941214872".parse().unwrap(),
            "0xfd59feD082378c9Ee4F57F62D1e10e2E95026c08".parse().unwrap(),
            "0x3A7315a05Bfca36CD309266F99028cF80AD6b1C6".parse().unwrap(),
            "0x46E715C0826123824352F4f0BCD279c815A0945E".parse().unwrap(),
            "0xeE553341d93bcF93e77E101e15bCbe07aF7E488f".parse().unwrap(),
            "0xD0875336db5a5b6FD70081918c559284Dc8434fA".parse().unwrap(),
        ],
    );
    map
});

// Create index for faster lookups
pub static ENABLED_MARKETS_INDEX: Lazy<HashMap<u32, HashMap<Address, bool>>> = Lazy::new(|| {
    ENABLED_MARKETS
        .iter()
        .map(|(&chain_id, addresses)| {
            let index = addresses
                .iter()
                .map(|&addr| (addr, true))
                .collect();
            (chain_id, index)
        })
        .collect()
});

// Check if market is enabled
pub fn is_market_enabled(chain_id: u32, market_address: Address) -> bool {
    ENABLED_MARKETS_INDEX
        .get(&chain_id)
        .and_then(|markets| markets.get(&market_address))
        .copied()
        .unwrap_or(false)
}

