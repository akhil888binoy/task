use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::model::token::Token;




// // Token-related types
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Token {
//     pub name: String,
//     pub market_type: Option<String>,
//     pub symbol: String,
//     pub asset_symbol: Option<String>,
//     pub base_symbol: Option<String>,
//     pub decimals: u32,
//     pub address: Address,
//     pub price_decimals: Option<u8>,
//     pub wrapped_address: Option<Address>,
//     pub coingecko_url: Option<String>,
//     pub coingecko_symbol: Option<String>,
//     pub metamask_symbol: Option<String>,
//     pub explorer_symbol: Option<String>,
//     pub explorer_url: Option<String>,
//     pub reserves_url: Option<String>,
//     pub image_url: Option<String>,
//     pub is_usdg: Option<bool>,
//     pub is_native: Option<bool>,
//     pub is_wrapped: Option<bool>,
//     pub is_shortable: Option<bool>,
//     pub is_stable: Option<bool>,
//     pub is_synthetic: Option<bool>,
//     pub is_temp_hidden: Option<bool>,
//     pub is_chart_disabled: Option<bool>,
//     pub is_v1_available: Option<bool>,
//     pub is_platform_token: Option<bool>,
//     pub is_platform_trading_token: Option<bool>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPrices {
    pub min_price: U256,
    pub max_price: U256,
}
pub type TokenPricesData = HashMap<Address, TokenPrices>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    #[serde(flatten)]
    pub token: Token,
    pub prices: TokenPrices,
    pub balance: Option<U256>,
    pub total_supply: Option<U256>,
}

pub type TokensData = HashMap<Address, TokenData>;

// Market-related types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub market_token_address: Address,
    pub index_token_address: Address,
    pub long_token_address: Address,
    pub short_token_address: Address,
    pub is_same_collaterals: bool,
    pub is_spot_only: bool,
    pub name: String,
    pub data: String,
    pub is_dynamic: Option<bool>,
}

pub type MarketsData = HashMap<Address, Market>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketInfo {
    #[serde(flatten)]
    pub market: Market,
    pub is_disabled: bool,
    pub long_token: TokenData,
    pub short_token: TokenData,
    pub index_token: TokenData,
    pub long_pool_amount: U256,
    pub short_pool_amount: U256,
    pub max_long_pool_amount: U256,
    pub max_short_pool_amount: U256,
    pub max_long_pool_amount_for_deposit: U256,
    pub max_short_pool_amount_for_deposit: U256,
    pub long_pool_amount_adjustment: U256,
    pub short_pool_amount_adjustment: U256,
    pub pool_value_max: U256,
    pub pool_value_min: U256,
    pub reserve_factor_long: U256,
    pub reserve_factor_short: U256,
    pub open_interest_reserve_factor_long: U256,
    pub open_interest_reserve_factor_short: U256,
    pub max_open_interest_long: U256,
    pub max_open_interest_short: U256,
    pub borrowing_factor_long: U256,
    pub borrowing_factor_short: U256,
    pub borrowing_exponent_factor_long: U256,
    pub borrowing_exponent_factor_short: U256,
    pub funding_factor: U256,
    pub funding_exponent_factor: U256,
    pub funding_increase_factor_per_second: U256,
    pub funding_decrease_factor_per_second: U256,
    pub threshold_for_stable_funding: U256,
    pub threshold_for_decrease_funding: U256,
    pub min_funding_factor_per_second: U256,
    pub max_funding_factor_per_second: U256,
    pub total_borrowing_fees: U256,
    pub position_impact_pool_amount: U256,
    pub min_position_impact_pool_amount: U256,
    pub position_impact_pool_distribution_rate: U256,
    pub min_collateral_factor: U256,
    pub min_collateral_factor_for_open_interest_long: U256,
    pub min_collateral_factor_for_open_interest_short: U256,
    pub swap_impact_pool_amount_long: U256,
    pub swap_impact_pool_amount_short: U256,
    pub max_pnl_factor_for_traders_long: U256,
    pub max_pnl_factor_for_traders_short: U256,
    pub pnl_long_min: U256,
    pub pnl_long_max: U256,
    pub pnl_short_min: U256,
    pub pnl_short_max: U256,
    pub net_pnl_min: U256,
    pub net_pnl_max: U256,
    pub claimable_funding_amount_long: Option<U256>,
    pub claimable_funding_amount_short: Option<U256>,
    pub long_interest_usd: U256,
    pub short_interest_usd: U256,
    pub long_interest_in_tokens: U256,
    pub short_interest_in_tokens: U256,
    pub position_fee_factor_for_positive_impact: U256,
    pub position_fee_factor_for_negative_impact: U256,
    pub position_impact_factor_positive: U256,
    pub position_impact_factor_negative: U256,
    pub max_position_impact_factor_positive: U256,
    pub max_position_impact_factor_negative: U256,
    pub max_position_impact_factor_for_liquidations: U256,
    pub position_impact_exponent_factor: U256,
    pub swap_fee_factor_for_positive_impact: U256,
    pub swap_fee_factor_for_negative_impact: U256,
    pub swap_impact_factor_positive: U256,
    pub swap_impact_factor_negative: U256,
    pub swap_impact_exponent_factor: U256,
    pub borrowing_factor_per_second_for_longs: U256,
    pub borrowing_factor_per_second_for_shorts: U256,
    pub funding_factor_per_second: U256,
    pub longs_pay_shorts: bool,
    pub virtual_pool_amount_for_long_token: U256,
    pub virtual_pool_amount_for_short_token: U256,
    pub virtual_inventory_for_positions: U256,
    pub virtual_market_id: Option<String>,
    pub virtual_long_token_id: Option<String>,
    pub virtual_short_token_id: Option<String>,
}

pub type MarketsInfoData = HashMap<Address, MarketInfo>;

// Dynamic market types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicMarket {
    pub market_token_address: Address,
    pub index_token_address: Vec<Address>,
    pub long_token_address: Address,
    pub short_token_address: Address,
    pub is_same_collaterals: bool,
    pub is_spot_only: bool,
    pub name: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMarketPrices {
    pub index_token_price: TokenPrices,
    pub long_token_price: TokenPrices,
    pub short_token_price: TokenPrices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMarketPricesForDynamicMarket {
    pub index_token_prices: Vec<TokenPrices>,
    pub long_token_price: TokenPrices,
    pub short_token_price: TokenPrices,
}

// Multicall types
#[derive(Debug, Clone)]
pub struct ContractCallConfig {
    pub method_name: String,
    pub params: Vec<String>, // Using String for simplicity, could use an enum
}

#[derive(Debug, Clone)]
pub struct ContractCallsConfig<T> {
    pub contract_address: Address,
    pub abi: String, // Would use proper ABI type in real implementation
    pub calls: T,
}

pub type MulticallRequestConfig<T> = HashMap<String, ContractCallsConfig<T>>;

#[derive(Debug, Clone)]
pub struct ContractCallResult {
    pub return_values: HashMap<String, String>, // Simplified
    pub contract_key: String,
    pub call_key: String,
    pub success: Option<bool>,
    pub error: Option<String>,
}

pub type MulticallResult = (
    bool, // success
    HashMap<String, HashMap<String, String>>, // errors
    HashMap<String, HashMap<String, ContractCallResult>>, // data
);

// Enum types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    Deposit,
    Withdrawal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Single,
    Pair,
}

// Constants
pub static EMPTY_OBJECT: Lazy<HashMap<(), ()>> = Lazy::new(HashMap::new);
