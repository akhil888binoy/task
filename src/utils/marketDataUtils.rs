use alloy::primitives::{U256, Address};
use std::cmp::{min, max};
use std::collections::HashMap;
use std::fmt;

#[path = "../types.rs"]
mod types;
#[path = "./hash.rs"]
mod hash;

use types::{MarketInfo, TokenData, TokenPrices};
use hash::expand_decimals;

// Constants
pub const PRECISION: U256 = expand_decimals(U256::from(1), 30);
pub const USD_DECIMALS: u32 = 30;
pub const MAX_EXCEEDING_THRESHOLD: &str = "1000000000";
pub const MIN_EXCEEDING_THRESHOLD: &str = "0.00000001";
pub const TRIGGER_PREFIX_ABOVE: &str = ">";
pub const TRIGGER_PREFIX_BELOW: &str = "<";

// Core Market Functions
pub fn get_max_pool_amount_for_deposit(market_info: &MarketInfo, is_long: bool) -> U256 {
    let (max_deposit, max_swap) = if is_long {
        (market_info.max_long_pool_amount_for_deposit, market_info.max_long_pool_amount)
    } else {
        (market_info.max_short_pool_amount_for_deposit, market_info.max_short_pool_amount)
    };
    min(max_deposit, max_swap)
}

pub fn get_deposit_collateral_capacity_amount(market_info: &MarketInfo, is_long: bool) -> U256 {
    let pool_amount = if is_long { market_info.long_pool_amount } else { market_info.short_pool_amount };
    let capacity = get_max_pool_amount_for_deposit(market_info, is_long).saturating_sub(pool_amount);
    max(capacity, U256::ZERO)
}

pub fn get_mid_price(prices: &TokenPrices) -> U256 {
    (prices.min_price + prices.max_price) / U256::from(2)
}

pub fn convert_to_usd(token_amount: U256, token_decimals: u32, price: U256) -> U256 {
    token_amount.checked_mul(price)
        .and_then(|v| v.checked_div(expand_decimals(U256::from(1), token_decimals)))
        .unwrap_or(U256::ZERO)
}

pub fn get_pool_usd_without_pnl(market_info: &MarketInfo, is_long: bool, price_type: &str) -> U256 {
    let token = if is_long { &market_info.long_token } else { &market_info.short_token };
    let price = match price_type {
        "minPrice" => token.prices.min_price,
        "maxPrice" => token.prices.max_price,
        _ => get_mid_price(&token.prices),
    };
    convert_to_usd(
        if is_long { market_info.long_pool_amount } else { market_info.short_pool_amount },
        token.decimals,
        price
    )
}

pub fn get_deposit_collateral_capacity_usd(market_info: &MarketInfo, is_long: bool) -> U256 {
    let capacity = get_max_pool_usd_for_deposit(market_info, is_long)
        .saturating_sub(get_pool_usd_without_pnl(market_info, is_long, "midPrice"));
    max(capacity, U256::ZERO)
}

pub fn convert_to_token_amount(usd: U256, token_decimals: u32, price: U256) -> Option<U256> {
    if price.is_zero() {
        return None;
    }
    usd.checked_mul(expand_decimals(U256::from(1), token_decimals))
        .and_then(|v| v.checked_div(price))
}

pub fn usd_to_market_token_amount(market_info: &MarketInfo, market_token: &TokenData, usd_value: U256) -> U256 {
    let supply = market_token.total_supply.unwrap_or_default();
    let pool_value = market_info.pool_value_max.unwrap_or_default();

    match (supply.is_zero(), pool_value.is_zero()) {
        (true, true) => convert_to_token_amount(usd_value, market_token.decimals, expand_decimals(U256::from(1), USD_DECIMALS))
            .unwrap_or_default(),
        (true, false) => convert_to_token_amount(usd_value + pool_value, market_token.decimals, expand_decimals(U256::from(1), USD_DECIMALS))
            .unwrap_or_default(),
        _ if pool_value.is_zero() => U256::ZERO,
        _ => supply.checked_mul(usd_value)
            .and_then(|v| v.checked_div(pool_value))
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct MintableMarketTokens {
    pub mintable_amount: U256,
    pub mintable_usd: U256,
    pub long_deposit_capacity_usd: U256,
    pub short_deposit_capacity_usd: U256,
    pub long_deposit_capacity_amount: U256,
    pub short_deposit_capacity_amount: U256,
}

pub fn get_mintable_market_tokens(market_info: &MarketInfo, market_token: &TokenData) -> MintableMarketTokens {
    let long_amount = get_deposit_collateral_capacity_amount(market_info, true);
    let short_amount = get_deposit_collateral_capacity_amount(market_info, false);
    
    let long_usd = get_deposit_collateral_capacity_usd(market_info, true);
    let short_usd = get_deposit_collateral_capacity_usd(market_info, false);
    
    let mintable_usd = long_usd + short_usd;
    let mintable_amount = usd_to_market_token_amount(market_info, market_token, mintable_usd);
    
    MintableMarketTokens {
        mintable_amount,
        mintable_usd,
        long_deposit_capacity_usd: long_usd,
        short_deposit_capacity_usd: short_usd,
        long_deposit_capacity_amount: long_amount,
        short_deposit_capacity_amount: short_amount,
    }
}

// Liquidity and Reserve Functions
pub fn get_available_usd_liquidity_for_collateral(market_info: &MarketInfo, is_long: bool) -> U256 {
    if market_info.is_spot_only {
        return get_pool_usd_without_pnl(market_info, is_long, "minPrice");
    }

    let reserved_usd = get_reserved_usd(market_info, is_long);
    let max_reserve_factor = if is_long { market_info.reserve_factor_long } else { market_info.reserve_factor_short };

    if max_reserve_factor.is_zero() {
        return U256::ZERO;
    }

    let min_pool_usd = reserved_usd.checked_mul(PRECISION)
        .and_then(|v| v.checked_div(max_reserve_factor))
        .unwrap_or(U256::MAX);

    get_pool_usd_without_pnl(market_info, is_long, "minPrice").saturating_sub(min_pool_usd)
}

pub fn get_reserved_usd(market_info: &MarketInfo, is_long: bool) -> U256 {
    if is_long {
        convert_to_usd(
            market_info.long_interest_in_tokens,
            market_info.index_token.decimals,
            market_info.index_token.prices.max_price
        )
    } else {
        market_info.short_interest_usd
    }
}

#[derive(Debug, Clone)]
pub struct SellableMarketToken {
    pub max_long_sellable_usd: U256,
    pub max_short_sellable_usd: U256,
    pub max_long_sellable_amount: U256,
    pub max_short_sellable_amount: U256,
    pub total_usd: U256,
    pub total_amount: U256,
}

pub fn get_sellable_market_token(market_info: &MarketInfo, market_token: &TokenData) -> SellableMarketToken {
    let long_pool_usd = convert_to_usd(
        market_info.long_pool_amount,
        market_info.long_token.decimals,
        market_info.long_token.prices.max_price
    );
    let short_pool_usd = convert_to_usd(
        market_info.short_pool_amount,
        market_info.short_token.decimals,
        market_info.short_token.prices.max_price
    );

    let long_liquidity = get_available_usd_liquidity_for_collateral(market_info, true);
    let short_liquidity = get_available_usd_liquidity_for_collateral(market_info, false);

    if long_pool_usd.is_zero() || short_pool_usd.is_zero() || long_liquidity.is_zero() || short_liquidity.is_zero() {
        return SellableMarketToken {
            max_long_sellable_usd: U256::ZERO,
            max_short_sellable_usd: U256::ZERO,
            max_long_sellable_amount: U256::ZERO,
            max_short_sellable_amount: U256::ZERO,
            total_usd: U256::ZERO,
            total_amount: U256::ZERO,
        };
    }

    let ratio = long_pool_usd.checked_mul(PRECISION)
        .and_then(|v| v.checked_div(short_pool_usd))
        .unwrap_or(PRECISION);

    let (max_long, max_short) = if short_liquidity.checked_mul(ratio)
        .and_then(|v| v.checked_div(PRECISION))
        .map_or(false, |v| v <= long_liquidity)
    {
        (
            short_liquidity.checked_mul(ratio)
                .and_then(|v| v.checked_div(PRECISION))
                .unwrap_or_default(),
            short_liquidity
        )
    } else {
        (
            long_liquidity,
            long_liquidity.checked_mul(PRECISION)
                .and_then(|v| v.checked_div(ratio))
                .unwrap_or_default()
        )
    };

    let max_long_amount = usd_to_market_token_amount(market_info, market_token, max_long);
    let max_short_amount = usd_to_market_token_amount(market_info, market_token, max_short);

    SellableMarketToken {
        max_long_sellable_usd: max_long,
        max_short_sellable_usd: max_short,
        max_long_sellable_amount: max_long_amount,
        max_short_sellable_amount: max_short_amount,
        total_usd: max_long + max_short,
        total_amount: max_long_amount + max_short_amount,
    }
}

// Formatting Utilities
pub fn limit_decimals(amount: &str, max_decimals: Option<usize>) -> String {
    let max_decimals = match max_decimals {
        Some(0) => return amount.split('.').next().unwrap_or(amount).to_string(),
        Some(d) => d,
        None => return amount.to_string(),
    };

    if let Some(dot_index) = amount.find('.') {
        let decimals = amount.len() - dot_index - 1;
        if decimals > max_decimals {
            return amount[..dot_index + max_decimals + 1].to_string();
        }
    }
    amount.to_string()
}

pub fn trim_zero_decimals(amount: &str) -> String {
    if amount.contains('.') {
        if let Ok(num) = amount.parse::<f64>() {
            if num.fract() == 0.0 {
                return num.to_i64().unwrap().to_string();
            }
        }
    }
    amount.to_string()
}

pub fn format_amount_free(amount: U256, token_decimals: u32, display_decimals: Option<usize>) -> String {
    let amount_str = format_units(amount, token_decimals);
    trim_zero_decimals(&limit_decimals(&amount_str, display_decimals))
}

pub fn format_units(value: U256, decimals: u32) -> String {
    let s = value.to_string();
    if decimals == 0 {
        return s;
    }

    let len = s.len();
    if len <= decimals as usize {
        format!("0.{}{}", "0".repeat(decimals as usize - len), s)
    } else {
        let split_pos = len - decimals as usize;
        format!("{}.{}", &s[..split_pos], &s[split_pos..])
    }
}

pub fn number_with_commas(x: &str) -> String {
    let parts: Vec<&str> = x.split('.').collect();
    let mut result = String::new();
    let int_part = parts[0];
    let len = int_part.len();
    
    for (i, c) in int_part.chars().enumerate() {
        if (len - i) % 3 == 0 && i != 0 {
            result.push(',');
        }
        result.push(c);
    }

    if parts.len() > 1 {
        result.push('.');
        result.push_str(parts[1]);
    }

    result
}

pub fn pad_decimals(amount: &str, min_decimals: usize) -> String {
    if let Some(dot_index) = amount.find('.') {
        let decimals = amount.len() - dot_index - 1;
        if decimals < min_decimals {
            return format!("{}{}", amount, "0".repeat(min_decimals - decimals));
        }
        amount.to_string()
    } else {
        format!("{}.{}", amount, "0".repeat(min_decimals))
    }
}

pub fn format_amount(
    amount: U256,
    token_decimals: u32,
    display_decimals: Option<usize>,
    use_commas: bool,
) -> String {
    let display_decimals = display_decimals.unwrap_or(2);
    let mut amount_str = format_units(amount, token_decimals);
    amount_str = limit_decimals(&amount_str, Some(display_decimals));
    
    if display_decimals != 0 {
        amount_str = pad_decimals(&amount_str, display_decimals);
    }
    
    if use_commas {
        number_with_commas(&amount_str)
    } else {
        amount_str
    }
}

#[derive(Debug)]
pub struct LimitedDisplay {
    pub symbol: &'static str,
    pub value: U256,
}

pub fn get_limited_display(amount: U256, token_decimals: u32) -> LimitedDisplay {
    let max = expand_decimals(U256::from_str(MAX_EXCEEDING_THRESHOLD).unwrap(), token_decimals);
    let min = expand_decimals(U256::from_str(MIN_EXCEEDING_THRESHOLD).unwrap(), token_decimals);
    let abs_amount = if amount < U256::ZERO { U256::ZERO - amount } else { amount };

    if abs_amount.is_zero() {
        return LimitedDisplay { symbol: "", value: abs_amount };
    }

    if abs_amount > max {
        LimitedDisplay { symbol: TRIGGER_PREFIX_ABOVE, value: max }
    } else if abs_amount < min {
        LimitedDisplay { symbol: TRIGGER_PREFIX_BELOW, value: min }
    } else {
        LimitedDisplay { symbol: "", value: abs_amount }
    }
}

pub fn format_token_amount(
    amount: Option<U256>,
    token_decimals: u32,
    symbol: Option<&str>,
    show_all_significant: bool,
    display_decimals: usize,
    use_commas: bool,
) -> Option<String> {
    let amount = amount?;
    let symbol_str = symbol.map(|s| format!(" {}", s)).unwrap_or_default();

    if show_all_significant {
        Some(format!("{}{}", format_amount_free(amount, token_decimals, Some(token_decimals as usize)), symbol_str))
    } else {
        let display = get_limited_display(amount, token_decimals);
        let amount_str = if display.symbol.is_empty() {
            format_amount(display.value, token_decimals, Some(display_decimals), use_commas)
        } else {
            format!("{} {}", display.symbol, format_amount(display.value, token_decimals, Some(display_decimals), use_commas))
        };
        Some(format!("{}{}", amount_str, symbol_str))
    }
}

// Additional Market Functions
pub fn get_max_reserved_usd(market_info: &MarketInfo, is_long: bool) -> U256 {
    let pool_usd = get_pool_usd_without_pnl(market_info, is_long, "minPrice");
    let mut reserve_factor = if is_long { market_info.reserve_factor_long } else { market_info.reserve_factor_short };
    let open_interest_factor = if is_long { market_info.open_interest_reserve_factor_long } else { market_info.open_interest_reserve_factor_short };

    if open_interest_factor < reserve_factor {
        reserve_factor = open_interest_factor;
    }

    pool_usd.checked_mul(reserve_factor)
        .and_then(|v| v.checked_div(PRECISION))
        .unwrap_or_default()
}

pub fn get_used_liquidity(market_info: &MarketInfo, is_long: bool) -> (U256, U256) {
    if market_info.is_spot_only {
        return (U256::ZERO, U256::ZERO);
    }

    let reserved_usd = get_reserved_usd(market_info, is_long);
    let max_reserved_usd = get_max_reserved_usd(market_info, is_long);
    let open_interest_usd = if is_long { market_info.long_interest_usd } else { market_info.short_interest_usd };
    let max_open_interest_usd = if is_long { market_info.max_open_interest_long } else { market_info.max_open_interest_short };

    let reserve_diff = max_reserved_usd.saturating_sub(reserved_usd);
    let interest_diff = max_open_interest_usd.saturating_sub(open_interest_usd);

    if reserve_diff < interest_diff {
        (reserved_usd, max_reserved_usd)
    } else {
        (open_interest_usd, max_open_interest_usd)
    }
}

// Helper functions
fn format_units(value: U256, decimals: u32) -> String {
    let s = value.to_string();
    if decimals == 0 {
        return s;
    }

    let len = s.len();
    if len <= decimals as usize {
        format!("0.{}{}", "0".repeat(decimals as usize - len), s)
    } else {
        let split_pos = len - decimals as usize;
        format!("{}.{}", &s[..split_pos], &s[split_pos..])
    }
}