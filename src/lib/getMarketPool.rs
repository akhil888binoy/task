use std::fmt;

#[path = "../types.rs"]
mod types;

#[path = "../utils/marketDataUtils.rs"]
mod market_data_utils;

use types::{MarketInfo, TokenData};
use market_data_utils::{
    convert_to_usd, 
    convert_to_token_amount, 
    get_mintable_market_tokens, 
    get_sellable_market_token,
    format_token_amount,
    format_amount,
};

pub struct MarketPoolData {
    pub pool_price_formatted: String,
}

pub async fn get_market_pool_data(market_info: Option<&MarketInfo>, market_token: Option<&TokenData>) -> Result<MarketPoolData, String> {
    // Validate inputs
    let market_info = match market_info {
        Some(info) => info,
        None => {
            log::warn!("marketInfo is undefined. Skipping pool data calculation.");
            return Err("marketInfo is undefined".to_string());
        }
    };

    let market_token = match market_token {
        Some(token) => token,
        None => {
            log::warn!("marketToken is undefined. Skipping pool data calculation.");
            return Err("marketToken is undefined".to_string());
        }
    };

    let pool_price = match market_token.prices.as_ref().and_then(|p| p.max_price) {
        Some(price) => price,
        None => {
            log::warn!("marketPrice is undefined for marketToken: {:?}", market_token);
            return Err("marketPrice is undefined".to_string());
        }
    };

    let long_token = match market_info.long_token.as_ref() {
        Some(token) => token,
        None => {
            log::warn!("Long token is undefined for marketInfo: {:?}", market_info);
            return Err("Long token is undefined".to_string());
        }
    };

    let short_token = match market_info.short_token.as_ref() {
        Some(token) => token,
        None => {
            log::warn!("Short token is undefined for marketInfo: {:?}", market_info);
            return Err("Short token is undefined".to_string());
        }
    };

    // Calculate mintable and sellable info
    let mintable_info = get_mintable_market_tokens(market_info, market_token);
    let sellable_info = get_sellable_market_token(market_info, market_token);
    let market_total_supply = market_token.total_supply;

    let _market_total_supply_usd = convert_to_usd(market_total_supply, market_token.decimals, pool_price);
    let market_balance = market_token.balance;

    let _market_balance_usd = convert_to_usd(market_balance, market_token.decimals, pool_price);

    // Format pool price
    let pool_price_formatted = format!("${}", format_amount(pool_price, 30));

    Ok(MarketPoolData {
        pool_price_formatted,
    })
}

trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl PrettyPrint for f64 {
    fn pretty_print(&self) -> String {
        format_amount(*self, 30)
    }
}