
use crate::config::chainConfig::BITLAYER;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use alloy::primitives::Address;

pub static ENABLED_MARKETS: Lazy<HashMap<u32, Vec<Address>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        BITLAYER,
        vec![
            "0xd3e7fB69556F1D3fa4ed83c18736573B31c118fA".parse().unwrap(),
            "0xA735F6b9FC36824ADe87347C8164D71dD067D7EA".parse().unwrap(),
            "0x140b79C9a6F501C87beD103C19AeD00B630d8d7C".parse().unwrap(),
            "0x5257D02BcF300d0DfE51470507E7cE91B0832c9A".parse().unwrap(),
            "0x5b356375451d023D1841C30A9ca8e8B41234Ef49".parse().unwrap(),
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

