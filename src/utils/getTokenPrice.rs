use reqwest;
use std::collections::HashMap;
use alloy::primitives::{Address, U256};
use serde::Deserialize;
use crate::{
    utils::{
        getToken::{get_token, get_wrapped_token},
        hash::parse_contract_price,
    },
    types::types::{TokenPricesData,TokenPrices},
};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct PriceItem {
    token_address: String,
    token_symbol:String, 
    min_price: String,
    max_price: String,
    updated_at: u64,
    price_decimals: u32,
}

#[derive(Debug, Deserialize)]
struct ResponsePriceItem{
    tokenAddress: String,
    tokenSymbol:String,
    minPrice: String,
    maxPrice: String,
    updatedAt: u64,
    priceDecimals: u32,
}
#[derive(Debug, Deserialize)]
pub struct TokenPricesDataResult {
    pub prices_data: Option<TokenPricesData>,
    pub updated_at: Option<u64>,
}

pub async fn fetch_token_recent_prices(chain_id: u32) -> Result<TokenPricesDataResult, reqwest::Error> {
    let oracle_fetcher_url = format!("{}/prices/tickers", "https://oracle.alphax.finance");
    let response = reqwest::get(&oracle_fetcher_url).await?;
     // If you know the structure, you can parse it into a specific type
     let body = response.text().await?;
     
     let price_items: Vec<ResponsePriceItem> = serde_json::from_str(&body).unwrap();

    //  let price_items:Vec<PriceItem> = vec![ 
    //     PriceItem {
    //         token_address: priceitems.tokenAddress,
    //         token_symbol:, 
    //         min_price: String,
    //         max_price: String,
    //         updated_at: u64,
    //         price_decimals: u32,
    //  }
    //  ];
    
     // Or if you want to handle it as generic JSON
     let json: Value = serde_json::from_str(&body).unwrap();

    // let price_items: Vec<PriceItem> = response.json().await?;

    let mut result: TokenPricesData = HashMap::new();

    for price_item in price_items {
        // Skip if prices are empty
       
        if price_item.minPrice.is_empty() || price_item.maxPrice.is_empty() {
            continue;
        }

        let token_address = match price_item.tokenAddress.parse::<Address>() {
            Ok(addr) => addr,
            Err(_) => continue, // Skip invalid addresses
        };

        let token_config = match get_token(chain_id, &token_address.to_string()) {
            Ok(config) => config,
            Err(e) => continue, // Skip unknown tokens
        };

       
        let min_price = U256::from_str_radix(price_item.minPrice.trim(), 10).unwrap();
        let max_price = U256::from_str_radix(price_item.maxPrice.trim(), 10).unwrap();

        // Store parsed prices
        result.insert(
            token_address,
            TokenPrices{
                min_price: parse_contract_price(min_price, token_config.decimals),
                max_price: parse_contract_price(max_price, token_config.decimals),
            }        
            ,
        );
    }

    // Handle wrapped token mapping
    if let Some(wrapped_token) = get_wrapped_token(chain_id) {
        if let Some(prices) = result.get(&wrapped_token.address) {
            if !result.contains_key(&Address::ZERO) {
                result.insert(Address::ZERO, prices.clone());
            }
        }
    }

    log::debug!("Processed Prices: {:?}", result);

    Ok(TokenPricesDataResult {
        prices_data: Some(result),
        updated_at: Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()),
    })
}