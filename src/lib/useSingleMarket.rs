use alloy::{
    primitives::{address, Address},
    providers::ProviderBuilder,
    sol,
};
use eyre::Result;
use std::path::Path;
use alloy::primitives::U256;


use crate::config::chainConfig::{create_bitlayer_provider, BITLAYER};
use crate::utils::getContracts::get_contract;
use crate::utils::hash::hash_string;
use crate::utils::marketEnabled::is_market_enabled;
use crate::utils::helpers::{get_market_full_name, MARKETS_COUNT , MarketNameParams};
use crate::types::types::MarketsData;
use crate::model::token::Token;
use crate::utils::getToken::get_token;
use std::collections::{HashMap, HashSet};

// Generate contract bindings
sol! {
    #[sol(rpc)]
    MarketReader,
    "/home/akhil888binoy/developer/taskassignment/rust-pool-stats/src/utils/abi/MarketReader.json"
}

sol! {
    #[sol(rpc)]
    DataStore,
    "/home/akhil888binoy/developer/taskassignment/rust-pool-stats/src/utils/abi/DataStore.json"
}

#[derive(Debug, Clone)]
pub struct MarketInfo {
    pub market_token: Address,
    pub index_token: Address,
    pub long_token: Address,
    pub short_token: Address,
}

// #[derive(Debug, Clone)]
// pub struct MarketNameParams1 {
//     pub long_token: Token,
//     pub short_token: Token,
//     pub index_token: Token,
//     pub is_spot_only: bool,
// }

#[derive(Debug, Clone)]
pub struct ProcessedMarket {
    pub market_token_address: Address,
    pub index_token_address: Address,
    pub long_token_address: Address,
    pub short_token_address: Address,
    pub is_same_collaterals: bool,
    pub is_spot_only: bool,
    pub name: String,
    pub data: String,
}

pub async fn use_single_markets(chain_id: u32) -> Result<(HashMap<String, ProcessedMarket>, Vec<String>)> {
    let provider = create_bitlayer_provider();
    let data_store_address = get_contract(chain_id, "DataStore")?;
    let market_reader_address = get_contract(chain_id, "MarketReader")?;

    let market_reader = MarketReader::new(market_reader_address, provider.clone());

    // Fetch markets in batches to handle large counts
    let batch_size = 50;
    let mut all_markets = Vec::new();
    
    for i in (0..MARKETS_COUNT).step_by(batch_size) {
        let end = std::cmp::min(i + batch_size as u32, MARKETS_COUNT);
        let markets = market_reader
            .getMarkets(data_store_address, U256::from(i), U256::from(end))
            .call()
            .await?;
        
        all_markets.extend(markets._0);
    }

    let mut processed_market_identifiers = HashSet::new();
    let mut markets_data = HashMap::new();
    let mut markets_addresses = Vec::new();

    for market in all_markets {
        let market_identifier = format!("{}-{}", market.marketToken, market.indexToken);
        
        // Skip already processed markets
        if processed_market_identifiers.contains(&market_identifier) {
            continue;
        }

        // Check if market is enabled
        if !is_market_enabled(chain_id, market.marketToken) {
            continue;
        }

        // Get token details
        let index_token = get_token(chain_id, &market.indexToken.to_string()).unwrap();
        let long_token = get_token(chain_id, &market.longToken.to_string()).unwrap();
        let short_token = get_token(chain_id, &market.shortToken.to_string()).unwrap();

        let is_same_collaterals = market.longToken == market.shortToken;
        let is_spot_only = market.indexToken == Address::ZERO;

        let market_params = MarketNameParams {
            long_token: long_token.clone(),
            short_token: short_token.clone(),
            index_token: index_token.clone(),
            is_spot_only,
        };

        let name = get_market_full_name(market_params);

        // Add to processed data
        markets_data.insert(
            market_identifier.clone(),
            ProcessedMarket {
                market_token_address: market.marketToken,
                index_token_address: market.indexToken,
                long_token_address: market.longToken,
                short_token_address: market.shortToken,
                is_same_collaterals,
                is_spot_only,
                name,
                data: String::new(),
            },
        );

        markets_addresses.push(market_identifier.clone());
        processed_market_identifiers.insert(market_identifier.clone());
    }
    println!("MARKETS DATA {:?} |||||| MARKETS ADDRESSES {:?}", markets_data , markets_addresses);
    Ok((markets_data, markets_addresses))
}

