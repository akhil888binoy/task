use std::collections::HashMap;
use alloy::primitives::{Address, U256};
use crate::types::types::{TokensData, TokenData, TokenPrices};
use crate::utils::getTokenPrice::fetch_token_recent_prices;
use crate::utils::getToken::{get_tokens_map, get_v2_tokens};
use crate::model::token::Token;

pub struct TokensDataResponse {
    pub tokens_data: Option<TokensData>,
    pub prices_updated_at: Option<u64>,
}

pub async fn use_tokens_data_request(chain_id: u32) -> Result<TokensDataResponse, Box<dyn std::error::Error>> {
    // Fetch token configurations
    let token_configs = match get_tokens_map(chain_id){
        Some(data)=>data,
        None=> panic!("not tokens map for this chain id")
    };
    let balance_data = Some(U256::from(0)); // Using 0 as BigInt equivalent
    let total_supply = None; // Assuming unknown unless provided


    // Fetch prices
    let price_result = match fetch_token_recent_prices(chain_id).await{
        Ok(data)=>data,
        Err(e)=>panic!("No Token recent prices")
    };

    let prices_data = match price_result.prices_data {
        Some(data) => data,
        None => {
            log::warn!("No Prices Data Found");
            return Ok(TokensDataResponse {
                tokens_data: None,
                prices_updated_at: None,
            });
        }
    };

    // Get V2 tokens
    let v2_tokens = match get_v2_tokens(chain_id){
        Some(data)=>data,
        None=> panic!("Not v2 tokens in this chain id")
    };

    log::debug!("V2 Tokens for chainId: {} - {:?}", chain_id, v2_tokens);

    let token_addresses: Vec<Address> = v2_tokens.iter()
        .map(|token| token.address)
        .collect();
    log::debug!("Valid Token Addresses: {:?}", token_addresses);

    let mut tokens_data = TokensData::new();



    for token_address in token_addresses.iter() {
        let prices = match prices_data.get(token_address) {
            Some(p) => p.clone(),
            None => {
                println!("Skipping token: {:?} (missing prices)", token_address);
                continue;
            }
        };

        let token_config = token_configs.get(&token_address.to_string())
        .or_else(|| token_configs.get(&token_address.to_string().to_lowercase()))
        .or_else(|| {
            token_configs.values().find(|config| 
                config.address == token_address.clone()
            )
        });

    let token_config = match token_config {
        Some(c) => c,
        None => {
            println!("Token config not found for {:?} (tried all methods)", token_address);
            continue;
        }
    };
        
        let token_entry = TokenData{
            token:token_config.clone(),
            prices: prices.clone(),
            balance : balance_data.clone(),
            total_supply:total_supply.clone()
        };

        tokens_data.insert(token_address.clone(), token_entry);
    }

    println!("TOKENS DATA : {:?}", tokens_data);

    log::debug!("Final Tokens Data: {:?}", tokens_data);

    Ok(TokensDataResponse {
        tokens_data: Some(tokens_data),
        prices_updated_at: price_result.updated_at,
    })
}