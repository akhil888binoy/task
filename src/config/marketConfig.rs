#[path = "./chainConfig.rs"]
mod chain_config;

use chain_config::MOVEMENT_DEVNET_CHAIN_ID;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use alloy::primitives::Address;

#[derive(Debug, Clone)]
pub struct MarketParams {
    pub market_token: Address,
    pub index_token: Address,
    pub long_token: Address,
    pub short_token: Address,
}

pub static SINGLE_MARKET_LIST: Lazy<HashMap<u32, Vec<Address>>> = Lazy::new(|| {
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

pub static SINGLE_MARKET_PROPS_DATA: Lazy<HashMap<u32, HashMap<Address, MarketParams>>> = Lazy::new(|| {
    let mut result = HashMap::new();
    let mut movement_markets = HashMap::new();

    // Market 1
    movement_markets.insert(
        "0xD27B8001E00da3018c604d98c1d3715941214872".parse().unwrap(),
        MarketParams {
            market_token: "0xD27B8001E00da3018c604d98c1d3715941214872".parse().unwrap(),
            index_token: "0xEb3c2e768c17E0c2AFF98bdF0024D38A18b0B62E".parse().unwrap(),
            long_token: "0x38604D543659121faa8F68A91A5b633C7BFE9761".parse().unwrap(),
            short_token: "0x38604D543659121faa8F68A91A5b633C7BFE9761".parse().unwrap(),
        },
    );

    // Market 2
    movement_markets.insert(
        "0xfd59feD082378c9Ee4F57F62D1e10e2E95026c08".parse().unwrap(),
        MarketParams {
            market_token: "0xfd59feD082378c9Ee4F57F62D1e10e2E95026c08".parse().unwrap(),
            index_token: "0xd778B815E6AE26f547042bbbe4Bf8b1B0c746A22".parse().unwrap(),
            long_token: "0x38604D543659121faa8F68A91A5b633C7BFE9761".parse().unwrap(),
            short_token: "0x38604D543659121faa8F68A91A5b633C7BFE9761".parse().unwrap(),
        },
    );

    // Market 3
    movement_markets.insert(
        "0x3A7315a05Bfca36CD309266F99028cF80AD6b1C6".parse().unwrap(),
        MarketParams {
            market_token: "0x3A7315a05Bfca36CD309266F99028cF80AD6b1C6".parse().unwrap(),
            index_token: "0xeAC3d56DCB15a3Bc174aB292B7023e9Fc9F7aDf0".parse().unwrap(),
            long_token: "0xeAC3d56DCB15a3Bc174aB292B7023e9Fc9F7aDf0".parse().unwrap(),
            short_token: "0x38604D543659121faa8F68A91A5b633C7BFE9761".parse().unwrap(),
        },
    );

    // Market 4
    movement_markets.insert(
        "0x46E715C0826123824352F4f0BCD279c815A0945E".parse().unwrap(),
        MarketParams {
            market_token: "0x46E715C0826123824352F4f0BCD279c815A0945E".parse().unwrap(),
            index_token: "0x2A197C29f3E144387EB5877CFe0e63032FD1a0DA".parse().unwrap(),
            long_token: "0x2A197C29f3E144387EB5877CFe0e63032FD1a0DA".parse().unwrap(),
            short_token: "0x38604D543659121faa8F68A91A5b633C7BFE9761".parse().unwrap(),
        },
    );

    // Market 5
    movement_markets.insert(
        "0xeE553341d93bcF93e77E101e15bCbe07aF7E488f".parse().unwrap(),
        MarketParams {
            market_token: "0xeE553341d93bcF93e77E101e15bCbe07aF7E488f".parse().unwrap(),
            index_token: "0x1AD94D0a799664D459cB467655eC0EA4cc8Ad478".parse().unwrap(),
            long_token: "0x1AD94D0a799664D459cB467655eC0EA4cc8Ad478".parse().unwrap(),
            short_token: "0x38604D543659121faa8F68A91A5b633C7BFE9761".parse().unwrap(),
        },
    );

    // Market 6
    movement_markets.insert(
        "0xD0875336db5a5b6FD70081918c559284Dc8434fA".parse().unwrap(),
        MarketParams {
            market_token: "0xD0875336db5a5b6FD70081918c559284Dc8434fA".parse().unwrap(),
            index_token: "0xb9aDf17948481eb380D37E9594fD4382372DBcd0".parse().unwrap(),
            long_token: "0xb9aDf17948481eb380D37E9594fD4382372DBcd0".parse().unwrap(),
            short_token: "0x38604D543659121faa8F68A91A5b633C7BFE9761".parse().unwrap(),
        },
    );

    result.insert(MOVEMENT_DEVNET_CHAIN_ID, movement_markets);
    result
});