use alloy::{
    network::Network,
    providers::{Provider, ProviderBuilder},
    transports::http::Http,
    primitives::{Address, Bytes},
    contract::Contract,
};
use serde::{Serialize, Deserialize};
use std::error::Error;

#[path = "../utils/getContracts.rs"]
mod get_contracts;
#[path = "../utils/hash.rs"]
mod hash;
#[path = "../utils/helpers.rs"]
mod helpers;
#[path = "../utils/marketEnabled.rs"]
mod market_enabled;

#[path="../config/chainConfig.rs"]
mod chainConfig;


use get_contracts::get_contract;
use hash::hash_string;
use market_enabled::is_market_enabled;
use helpers::MARKETS_COUNT;
use chainConfig::{MOVEMENT_DEVNET_CHAIN_ID , create_movement_provider};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketLists {
    pub single_markets: Vec<Address>,
    pub dynamic_markets: Vec<Address>,
}

pub async fn get_markets() -> Result<MarketLists, Box<dyn Error>> {
    // Use your pre-configured provider
    let provider = create_movement_provider();
    
    // Get DataStore contract using your chain configuration
    let contract_address = get_contract(MOVEMENT_DEVNET_CHAIN_ID , "DataStore")?;
    let abi_bytes = include_bytes!("../utils/abi/DataStore.json");
    let data_store = Contract::from_json(contract_address, abi_bytes)?;

    // Rest of your existing implementation...
    let single_market_call = data_store
        .method::<_, Vec<Address>>(
            "getAddressValuesAt", 
            (hash_string("SINGLE_MARKET_LIST"), 0_u64, MARKETS_COUNT)
        )?;

    let dynamic_market_call = data_store
        .method::<_, Vec<Address>>(
            "getAddressValuesAt", 
            (hash_string("DYNAMIC_MARKET_LIST"), 0_u64, MARKETS_COUNT)
        )?;

    let single_markets_result = single_market_call.call().await?;
    let dynamic_markets_result = dynamic_market_call.call().await?;

    let single_markets = single_markets_result
        .into_iter()
        .filter(|&address| is_market_enabled(MOVEMENT_DEVNET_CHAIN_ID , address))
        .collect();

    let dynamic_markets = dynamic_markets_result
        .into_iter()
        .filter(|&address| is_market_enabled(MOVEMENT_DEVNET_CHAIN_ID , address))
        .collect();

    Ok(MarketLists {
        single_markets,
        dynamic_markets,
    })
}