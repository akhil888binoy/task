use alloy::{
    primitives::{address, Address},
    providers::ProviderBuilder,
    sol,
};
use eyre::Result;
use std::path::Path;
use alloy::primitives::U256;


use crate::config::chainConfig::{create_bitlayer_provider, BITLAYER};
use crate::utils::marketEnabled::is_market_enabled;
use crate::utils::getContracts::get_contract;
use crate::utils::hash::hash_string;
use crate::utils::helpers::MARKETS_COUNT;

// Generate contract bindings from ABI file
sol!(
    #[sol(rpc)]
    DataStore,
    "/home/akhil888binoy/developer/taskassignment/rust-pool-stats/src/utils/abi/DataStore.json"  // Path to your ABI file
);

#[derive(Debug, Clone)]
pub struct MarketLists {
    pub single_markets: Vec<Address>,
    pub dynamic_markets: Vec<Address>,
}

pub async fn get_markets() -> Result<MarketLists> {
    let provider = create_bitlayer_provider();
    let contract_address = match get_contract(BITLAYER, "DataStore"){
        Ok(data)=>data,
        Err(e)=> panic!("Error DataStore not found")
    };

    println!("CONTRACT ADDRESS : {}", contract_address);

    // Create contract instance
    let data_store = DataStore::new(contract_address, provider);
    println!("DATA STORE : {:?}", data_store);

    // Make contract calls
    let single_market_key = hash_string("SINGLE_MARKET_LIST");
    let dynamic_market_key = hash_string("DYNAMIC_MARKET_LIST");

    println!("WORKING TILL HERE 11111");
    println!("KEY : {}", single_market_key);


    let single_markets_result = data_store
    .getAddressValuesAt(
        single_market_key, 
        U256::from(0),
        U256::from(MARKETS_COUNT)  
    )
    .call()
    .await?
    ._0;

    println!("WORKING TILL HERE");
    println!("SINGLE MARKET RESULT : {:?}", single_markets_result);

    let dynamic_markets_result = data_store
        .getAddressValuesAt(dynamic_market_key,  U256::from(0), 
        U256::from(MARKETS_COUNT) )
        .call()
        .await?
        ._0;

    // Filter enabled markets
    let single_markets = single_markets_result
        .into_iter()
        .filter(|&address| is_market_enabled(BITLAYER, address))
        .collect();

    let dynamic_markets = dynamic_markets_result
        .into_iter()
        .filter(|&address| is_market_enabled(BITLAYER, address))
        .collect();

    let markets = MarketLists {
        single_markets,
        dynamic_markets,
    };

    Ok(markets)
}