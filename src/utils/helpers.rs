use alloy::primitives::{Address, U256};
use crate::utils::getToken::WRAPPED_TOKENS_MAP;
use crate::types::types::{
    ContractMarketPrices, ContractMarketPricesForDynamicMarket, DynamicMarket, Market, TokenData,
    TokenPrices, TokensData,
};

use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::model::token::Token;


pub const MARKETS_COUNT: u32 = 10;
pub const NATIVE_TOKEN_ADDRESS: Address = Address::ZERO;
pub static BN_ONE: Lazy<U256> = Lazy::new(|| U256::from(1));

#[derive(Clone,Debug)]
pub struct MarketNameParams {
    pub long_token: Token,
    pub short_token: Token,
    pub index_token: Token,
    pub is_spot_only: bool,
}

pub fn get_market_full_name(p: MarketNameParams) -> String {
    let index_name = get_market_index_name(&p.index_token, p.is_spot_only);
    let pool_name = get_market_pool_name(&p.long_token, &p.short_token);
    format!("{} [{}]", index_name, pool_name)
}

pub fn get_market_index_name(index_token: &Token, is_spot_only: bool) -> String {
    if is_spot_only {
        return "SWAP-ONLY".to_string();
    }

    match &index_token.base_symbol {
        Some(base_symbol) => format!("{}/USD", base_symbol),
        None => format!("{}/USD", index_token.symbol),
    }
}

pub fn get_market_pool_name(long_token: &Token, short_token: &Token) -> String {
    if long_token.address == short_token.address {
        long_token.symbol.clone()
    } else {
        format!("{}-{}", long_token.symbol, short_token.symbol)
    }
}

pub fn get_by_key<T: Clone>(obj: &HashMap<Address, T>, key: &Address) -> Option<T> {
    obj.get(key).cloned()
}

pub fn get_contract_market_prices(
    tokens_data: &TokensData,
    market: &Market,
) -> Option<ContractMarketPrices> {
    let index_token = get_by_key(tokens_data, &market.index_token_address)?;
    let long_token = get_by_key(tokens_data, &market.long_token_address)?;
    let short_token = get_by_key(tokens_data, &market.short_token_address)?;

    Some(ContractMarketPrices {
        index_token_price: convert_to_contract_token_prices(&index_token.prices, index_token.token.decimals),
        long_token_price: convert_to_contract_token_prices(&long_token.prices, long_token.token.decimals),
        short_token_price: convert_to_contract_token_prices(&short_token.prices, short_token.token.decimals),
    })
}

pub fn serialize_bigint<T: serde::Serialize>(obj: &T) -> serde_json::Result<String> {
    serde_json::to_string(obj)
}

pub fn get_wrapped_token(chain_id: u32) -> Option<&'static Token> {
    WRAPPED_TOKENS_MAP.get(&chain_id)
}

pub fn convert_token_address(
    chain_id: u32,
    address: Address,
    convert_to: Option<&str>,
) -> Address {
    if let Some(wrapped_token) = get_wrapped_token(chain_id) {
        match convert_to {
            Some("wrapped") if address == NATIVE_TOKEN_ADDRESS => return wrapped_token.address,
            Some("native") if address == wrapped_token.address => return NATIVE_TOKEN_ADDRESS,
            _ => {}
        }
    }
    address
}

pub fn convert_to_contract_price(price: U256, token_decimals: u32) -> U256 {
    price.checked_div(expand_decimals(U256::from(1), token_decimals))
        .unwrap_or(U256::ZERO)
}

pub fn convert_to_contract_token_prices(prices: &TokenPrices, token_decimals: u32) -> TokenPrices {
    TokenPrices {
        min_price: convert_to_contract_price(prices.min_price, token_decimals),
        max_price: convert_to_contract_price(prices.max_price, token_decimals),
    }
}

pub fn expand_decimals(n: U256, decimals: u32) -> U256 {
    n.checked_mul(U256::from(10).pow(U256::from(decimals)))
        .unwrap_or(U256::MAX)
}

pub fn get_contract_market_price_for_dynamic_market(
    tokens_data: &TokensData,
    market: &DynamicMarket,
) -> Option<ContractMarketPricesForDynamicMarket> {
    let mut index_tokens = Vec::new();

    for index_token_address in &market.index_token_address {
        if let Some(token_data) = get_by_key(tokens_data, &index_token_address) {
            index_tokens.push(token_data);
        } else {
            return None;
        }
    }

    let long_token = get_by_key(tokens_data, &market.long_token_address)?;
    let short_token = get_by_key(tokens_data, &market.short_token_address)?;

    let index_token_prices = index_tokens
        .iter()
        .map(|token| convert_to_contract_token_prices(&token.prices, token.token.decimals))
        .collect();

    Some(ContractMarketPricesForDynamicMarket {
        index_token_prices,
        long_token_price: convert_to_contract_token_prices(&long_token.prices, long_token.token.decimals),
        short_token_price: convert_to_contract_token_prices(&short_token.prices, short_token.token.decimals),
    })
}