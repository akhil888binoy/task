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
    let price_result = fetch_token_recent_prices(chain_id).await?;
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

    for token_address in token_addresses {
        let prices = match prices_data.get(&token_address) {
            Some(p) => p.clone(),
            None => {
                log::warn!("Skipping token: {:?} (missing prices)", token_address);
                continue;
            }
        };

        let token_config = match token_configs.get(&token_address.to_string()){
            Some(data)=> data,
            None=>continue
        };

        println!("TOKEN CONFIG:{:?}", token_config);
        
        let token_entry = match token_config {
            Some(config) => {

                
                let tokendata = TokenData {
                    token: Token {
                        name: config.token.name.clone(),
                        symbol: config.token.symbol.clone(),
                        decimals: config.token.decimals,
                        address: token_address,
                        ..Default::default()
                    },
                    prices,
                    balance: balance_data,
                    total_supply: total_supply,
                };
                tokendata

            },
            None => {
                log::warn!(
                    "Missing tokenConfig for address {:?}. Constructing fallback.",
                    token_address
                );

                TokenData {
                    token: Token {
                        name: format!("Unknown Token ({})", &token_address.to_string()[..8]),
                        symbol: "UNKNOWN".to_string(),
                        decimals: 18,
                        address: token_address,
                        ..Default::default() 
                    },
                    prices,
                    balance: balance_data,
                    total_supply,
                }
            }
        };

        tokens_data.insert(token_address, token_entry);
    }

    log::debug!("Final Tokens Data: {:?}", tokens_data);

    Ok(TokensDataResponse {
        tokens_data: Some(tokens_data),
        prices_updated_at: price_result.updated_at,
    })
}