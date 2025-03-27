use alloy::primitives::{U256, Address};
use std::cmp::{min, max};
use once_cell::sync::Lazy;
use crate::types::types::{MarketInfo, TokenData, TokenPrices};
use crate::utils::hash::expand_decimals;

// Constants
pub static PRECISION: Lazy<U256> = Lazy::new(|| expand_decimals(U256::from(1), 30));
pub static USD_DECIMALS: u32 = 30;
pub static MAX_EXCEEDING_THRESHOLD: Lazy<U256> = Lazy::new(|| U256::from(1_000_000_000u64));
pub static MIN_EXCEEDING_THRESHOLD: Lazy<U256> = Lazy::new(||{ U256::from(1u64)});
pub static TRIGGER_PREFIX_ABOVE: &str = ">";
pub static TRIGGER_PREFIX_BELOW: &str = "<";

// Pool amount calculations
pub fn get_max_pool_amount_for_deposit(market_info: &MarketInfo, is_long: bool) -> U256 {
    let max_amount_for_deposit = if is_long {
        market_info.max_long_pool_amount_for_deposit
    } else {
        market_info.max_short_pool_amount_for_deposit
    };
    
    let max_amount_for_swap = if is_long {
        market_info.max_long_pool_amount
    } else {
        market_info.max_short_pool_amount
    };

    min(max_amount_for_deposit, max_amount_for_swap)
}

pub fn get_deposit_collateral_capacity_amount(market_info: &MarketInfo, is_long: bool) -> U256 {
    let pool_amount = if is_long {
        market_info.long_pool_amount
    } else {
        market_info.short_pool_amount
    };
    
    let max_pool_amount = get_max_pool_amount_for_deposit(market_info, is_long);
    
    if max_pool_amount > pool_amount {
        max_pool_amount - pool_amount
    } else {
        U256::ZERO
    }
}

pub fn get_deposit_collateral_capacity_usd(market_info: &MarketInfo, is_long: bool) -> U256 {
    let pool_usd = get_pool_usd_without_pnl(market_info, is_long, "midPrice");
    let max_pool_usd = get_max_pool_usd_for_deposit(market_info, is_long);
    
    if max_pool_usd > pool_usd {
        max_pool_usd - pool_usd
    } else {
        U256::ZERO
    }
}

// Price calculations
pub fn get_mid_price(prices: &TokenPrices) -> U256 {
    (prices.min_price + prices.max_price) / U256::from(2)
}

pub fn convert_to_usd(
    token_amount: Option<U256>,
    token_decimals: Option<u32>,
    price: Option<U256>
) -> Option<U256> {
    match (token_amount, token_decimals, price) {
        (Some(amount), Some(decimals), Some(price)) => {
            Some((amount * price) / expand_decimals(U256::from(1), decimals))
        }
        _ => None,
    }
}

pub fn get_max_pool_usd_for_deposit(market_info: &MarketInfo, is_long: bool) -> U256 {
    let token = if is_long {
        &market_info.long_token
    } else {
        &market_info.short_token
    };
    
    let max_pool_amount = get_max_pool_amount_for_deposit(market_info, is_long);
    
    convert_to_usd(Some(max_pool_amount), Some(token.token.decimals), Some(get_mid_price(&token.prices)))
        .unwrap_or(U256::ZERO)
}

pub fn get_pool_usd_without_pnl(
    market_info: &MarketInfo,
    is_long: bool,
    price_type: &str,
) -> U256 {
    let pool_amount = if is_long {
        market_info.long_pool_amount
    } else {
        market_info.short_pool_amount
    };
    
    let token = if is_long {
        &market_info.long_token
    } else {
        &market_info.short_token
    };
    
    let price = match price_type {
        "minPrice" => token.prices.min_price,
        "maxPrice" => token.prices.max_price,
        _ => get_mid_price(&token.prices),
    };
    
    convert_to_usd(Some(pool_amount), Some(token.token.decimals), Some(price))
        .unwrap_or(U256::ZERO)
}

// Math utilities
pub struct BigMath;

impl BigMath {
    pub fn abs(x: U256) -> U256 {
        if x < U256::ZERO { U256::ZERO - x } else { x }
    }
    
    pub fn mul_div(x: U256, y: U256, z: U256) -> U256 {
        (x * y) / z
    }
}

pub fn convert_to_token_amount(
    usd: Option<U256>,
    token_decimals: Option<u32>,
    price: Option<U256>,
) -> Option<U256> {
    match (usd, token_decimals, price) {
        (Some(usd), Some(decimals), Some(price)) if price > U256::ZERO => {
            Some((usd * expand_decimals(U256::from(1), decimals)) / price)
        }
        _ => None,
    }
}

// Market token calculations
pub fn usd_to_market_token_amount(
    market_info: &MarketInfo,
    market_token: &TokenData,
    usd_value: U256,
) -> U256 {
    let supply = market_token.total_supply.unwrap_or(U256::ZERO);
    let pool_value = market_info.pool_value_max;
    
    if supply == U256::ZERO && pool_value == U256::ZERO {
        return convert_to_token_amount(
            Some(usd_value),
            Some(market_token.token.decimals),
            Some(expand_decimals(U256::from(1), USD_DECIMALS))
        ).unwrap_or(U256::ZERO);
    }
    
    if supply == U256::ZERO && pool_value > U256::ZERO {
        return convert_to_token_amount(
            Some(usd_value + pool_value),
            Some(market_token.token.decimals),
            Some(expand_decimals(U256::from(1), USD_DECIMALS))
        ).unwrap_or(U256::ZERO);
    }
    
    if pool_value == U256::ZERO {
        return U256::ZERO;
    }
    
    BigMath::mul_div(supply, usd_value, pool_value)
}

pub fn get_mintable_market_tokens(
    market_info: &MarketInfo,
    market_token: &TokenData,
) -> MintableMarketTokens {
    let long_deposit_capacity_amount = get_deposit_collateral_capacity_amount(market_info, true);
    let short_deposit_capacity_amount = get_deposit_collateral_capacity_amount(market_info, false);
    
    let long_deposit_capacity_usd = get_deposit_collateral_capacity_usd(market_info, true);
    let short_deposit_capacity_usd = get_deposit_collateral_capacity_usd(market_info, false);
    
    let mintable_usd = long_deposit_capacity_usd + short_deposit_capacity_usd;
    let mintable_amount = usd_to_market_token_amount(market_info, market_token, mintable_usd);
    
    MintableMarketTokens {
        mintable_amount,
        mintable_usd,
        long_deposit_capacity_usd,
        short_deposit_capacity_usd,
        long_deposit_capacity_amount,
        short_deposit_capacity_amount,
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

// Liquidity calculations
pub fn get_available_usd_liquidity_for_collateral(
    market_info: &MarketInfo,
    is_long: bool,
) -> U256 {
    let pool_usd = get_pool_usd_without_pnl(market_info, is_long, "minPrice");
    
    if market_info.market.is_spot_only {
        return pool_usd;
    }
    
    let reserved_usd = get_reserved_usd(market_info, is_long);
    let max_reserve_factor = if is_long {
        market_info.reserve_factor_long
    } else {
        market_info.reserve_factor_short
    };
    
    if max_reserve_factor == U256::ZERO {
        return U256::ZERO;
    }
    
    let min_pool_usd = (reserved_usd * *PRECISION) / max_reserve_factor;
    
    if pool_usd > min_pool_usd {
        pool_usd - min_pool_usd
    } else {
        U256::ZERO
    }
}

pub fn get_reserved_usd(market_info: &MarketInfo, is_long: bool) -> U256 {
    if is_long {
        convert_to_usd(
            Some(market_info.long_interest_in_tokens),
            Some(market_info.index_token.token.decimals),
            Some(market_info.index_token.prices.max_price)
        ).unwrap_or(U256::ZERO)
    } else {
        market_info.short_interest_usd
    }
}

// Sellable calculations
pub fn get_sellable_market_token(
    market_info: &MarketInfo,
    market_token: &TokenData,
) -> SellableMarketToken {
    let long_pool_usd = convert_to_usd(
        Some(market_info.long_pool_amount),
        Some(market_info.long_token.token.decimals),
        Some(market_info.long_token.prices.max_price)
    ).unwrap_or(U256::ZERO);
    
    let short_pool_usd = convert_to_usd(
        Some(market_info.short_pool_amount),
        Some(market_info.short_token.token.decimals),
        Some(market_info.short_token.prices.max_price)
    ).unwrap_or(U256::ZERO);
    
    let long_collateral_liquidity_usd = get_available_usd_liquidity_for_collateral(market_info, true);
    let short_collateral_liquidity_usd = get_available_usd_liquidity_for_collateral(market_info, false);
    
    let factor = expand_decimals(U256::from(1), 8);
    
    if long_pool_usd == U256::ZERO ||
       short_pool_usd == U256::ZERO ||
       long_collateral_liquidity_usd == U256::ZERO ||
       short_collateral_liquidity_usd == U256::ZERO
    {
        return SellableMarketToken {
            max_long_sellable_usd: U256::ZERO,
            max_short_sellable_usd: U256::ZERO,
            max_long_sellable_amount: U256::ZERO,
            max_short_sellable_amount: U256::ZERO,
            total_usd: U256::ZERO,
            total_amount: U256::ZERO,
        };
    }
    
    let ratio = BigMath::mul_div(long_pool_usd, factor, short_pool_usd);
    let (max_long_sellable_usd, max_short_sellable_usd) = 
        if BigMath::mul_div(short_collateral_liquidity_usd, ratio, factor) <= long_collateral_liquidity_usd {
            (
                BigMath::mul_div(short_collateral_liquidity_usd, ratio, factor),
                short_collateral_liquidity_usd
            )
        } else {
            (
                long_collateral_liquidity_usd,
                (long_collateral_liquidity_usd / ratio) * factor
            )
        };
    
    let max_long_sellable_amount = usd_to_market_token_amount(
        market_info,
        market_token,
        max_long_sellable_usd
    );
    
    let max_short_sellable_amount = usd_to_market_token_amount(
        market_info,
        market_token,
        max_short_sellable_usd
    );
    
    SellableMarketToken {
        max_long_sellable_usd,
        max_short_sellable_usd,
        max_long_sellable_amount,
        max_short_sellable_amount,
        total_usd: max_long_sellable_usd + max_short_sellable_usd,
        total_amount: max_long_sellable_amount + max_short_sellable_amount,
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

// Formatting utilities
pub fn limit_decimals(amount: &str, max_decimals: Option<usize>) -> String {
    if max_decimals.is_none() {
        return amount.to_string();
    }
    
    let max_decimals = max_decimals.unwrap();
    if max_decimals == 0 {
        return amount.split('.').next().unwrap_or("0").to_string();
    }
    
    if let Some(dot_index) = amount.find('.') {
        let decimals = amount.len() - dot_index - 1;
        if decimals > max_decimals {
            return amount[..dot_index + max_decimals + 1].to_string();
        }
    }
    
    amount.to_string()
}

pub fn trim_zero_decimals(amount: &str) -> String {
    if let Ok(num) = amount.parse::<f64>() {
        if num.fract() == 0.0 {
            return num.trunc().to_string();
        }
    }
    amount.to_string()
}

pub fn format_amount_free(
    amount: Option<U256>,
    token_decimals: u32,
    display_decimals: Option<usize>,
) -> String {
    if amount.is_none() {
        return "...".to_string();
    }
    
    let amount_str = format_units(amount.unwrap(), token_decimals);
    limit_decimals(&amount_str, display_decimals)
}

pub fn get_limited_display(
    amount: U256,
    token_decimals: u32,
    opts: LimitedDisplayOptions,
) -> LimitedDisplayResult {
    let max = expand_decimals(opts.max_threshold.unwrap_or(*MAX_EXCEEDING_THRESHOLD), token_decimals);
    let min = parse_units(opts.min_threshold.unwrap_or(*MIN_EXCEEDING_THRESHOLD) , token_decimals);
    
    let abs_amount = BigMath::abs(amount);
    
    if abs_amount == U256::ZERO {
        return LimitedDisplayResult {
            symbol: "".to_string(),
            value: abs_amount,
        };
    }
    
    let (symbol, value) = if abs_amount > max {
        (TRIGGER_PREFIX_ABOVE.to_string(), max)
    } else if abs_amount < min {
        (TRIGGER_PREFIX_BELOW.to_string(), min)
    } else {
        ("".to_string(), abs_amount)
    };
    
    LimitedDisplayResult { symbol, value }
}

pub fn number_with_commas(x: Option<U256>) -> String {
    match x {
        None => "...".to_string(),
        Some(num) => {
            // Store the string in a variable to extend its lifetime
            let num_str = num.to_string();
            let mut parts = num_str.split('.');
            let integer_part = parts.next().unwrap();
            let decimal_part = parts.next().unwrap_or("");
            
            let mut result = String::new();
            let mut count = 0;
            
            // Build the integer part with commas
            for c in integer_part.chars().rev() {
                if count != 0 && count % 3 == 0 {
                    result.insert(0, ',');
                }
                result.insert(0, c);
                count += 1;
            }
            
            // Add decimal part if exists
            if !decimal_part.is_empty() {
                result.push('.');
                result.push_str(decimal_part);
            }
            
            result
        }
    }
}

pub fn pad_decimals(amount: &str, min_decimals: usize) -> String {
    let mut amount_str = amount.to_string();
    
    if let Some(dot_index) = amount_str.find('.') {
        let decimals = amount_str.len() - dot_index - 1;
        if decimals < min_decimals {
            amount_str.push_str(&"0".repeat(min_decimals - decimals));
        }
    } else {
        amount_str.push_str(".");
        amount_str.push_str(&"0".repeat(min_decimals));
    }
    
    amount_str
}

pub fn format_amount(
    amount: Option<U256>,
    token_decimals: u32,
    display_decimals: Option<usize>,
    use_commas: Option<bool>,
    default_value: Option<&str>,
) -> String {
    let default_value = default_value.unwrap_or("...");
    
    if amount.is_none() {
        return default_value.to_string();
    }
    
    let display_decimals = display_decimals.unwrap_or(2);
    let mut amount_str = format_units(amount.unwrap(), token_decimals);
    amount_str = limit_decimals(&amount_str, Some(display_decimals));
    
    if display_decimals != 0 {
        amount_str = pad_decimals(&amount_str, display_decimals);
    }
    
    if use_commas.unwrap_or(false) {
        number_with_commas(amount_str.parse::<U256>().ok())
    } else {
        amount_str
    }

}

pub fn format_token_amount(
    amount: Option<U256>,
    token_decimals: Option<u32>,
    symbol: Option<&str>,
    opts: FormatTokenAmountOptions,
) -> Option<String> {
    let symbol_str = symbol.map(|s| format!(" {}", s)).unwrap_or_default();
    
    let (amount, token_decimals) = match (amount, token_decimals) {
        (Some(amount), Some(decimals)) => (amount, decimals),
        _ if opts.fallback_to_zero => (U256::ZERO, opts.display_decimals.unwrap_or(4) as u32),
        _ => return None,
    };
    
    let maybe_plus = if opts.display_plus { "+" } else { "" };
    let sign = if amount < U256::ZERO { "-" } else { maybe_plus };
    
    let amount_str = if opts.show_all_significant {
        format_amount_free(Some(amount), token_decimals, Some(token_decimals as usize))
    } else {
        let exceeding_info = get_limited_display(
            amount,
            token_decimals,
            LimitedDisplayOptions {
                max_threshold: opts.max_threshold,
                min_threshold: opts.min_threshold,
            },
        );
        let symbol = if !exceeding_info.symbol.is_empty() {
            format!("{} ", exceeding_info.symbol)
        } else {
            "".to_string()
        };
        format!(
            "{}{}{}",
            symbol,
            sign,
            format_amount(
                Some(exceeding_info.value),
                token_decimals,
                opts.display_decimals,
                opts.use_commas,
                None
            )
        )
    };
    
    Some(format!("{}{}", amount_str, symbol_str))
}

// Helper functions
fn format_units(value: U256, decimals: u32) -> String {
    // Simplified implementation - you may want to use a more complete one
    let divisor = U256::from(10).pow(U256::from(decimals));
    let integer = value / divisor;
    let fractional = value % divisor;
    
    if fractional == U256::ZERO {
        integer.to_string()
    } else {
        format!("{}.{}", integer, fractional)
    }
}

pub fn parse_units(value: U256, decimals: u32) -> U256 {
    value * U256::from(10).pow(U256::from(decimals))
}

#[derive(Debug, Clone, Default)]
pub struct LimitedDisplayOptions {
    pub max_threshold: Option<U256>,
    pub min_threshold: Option<U256>,
}

#[derive(Debug, Clone)]
pub struct LimitedDisplayResult {
    pub symbol: String,
    pub value: U256,
}

#[derive(Debug, Clone, Default)]
pub struct FormatTokenAmountOptions {
    pub show_all_significant: bool,
    pub display_decimals: Option<usize>,
    pub fallback_to_zero: bool,
    pub use_commas: Option<bool>,
    pub min_threshold: Option<U256>,
    pub max_threshold: Option<U256>,
    pub display_plus: bool,
}

// Additional market functions
pub fn get_max_reserved_usd(market_info: &MarketInfo, is_long: bool) -> U256 {
    let pool_usd = get_pool_usd_without_pnl(market_info, is_long, "minPrice");
    
    let mut reserve_factor = if is_long {
        market_info.reserve_factor_long
    } else {
        market_info.reserve_factor_short
    };
    
    let open_interest_reserve_factor = if is_long {
        market_info.open_interest_reserve_factor_long
    } else {
        market_info.open_interest_reserve_factor_short
    };
    
    if open_interest_reserve_factor < reserve_factor {
        reserve_factor = open_interest_reserve_factor;
    }
    
    (pool_usd * reserve_factor) / *PRECISION
}

pub fn get_open_interest_usd(market_info: &MarketInfo, is_long: bool) -> U256 {
    if is_long {
        market_info.long_interest_usd
    } else {
        market_info.short_interest_usd
    }
}

pub fn get_max_open_interest_usd(market_info: &MarketInfo, is_long: bool) -> U256 {
    if is_long {
        market_info.max_open_interest_long
    } else {
        market_info.max_open_interest_short
    }
}

pub fn get_used_liquidity(market_info: &MarketInfo, is_long: bool) -> (U256, U256) {
    if market_info.market.is_spot_only {
        return (U256::ZERO, U256::ZERO);
    }
    
    let reserved_usd = get_reserved_usd(market_info, is_long);
    let max_reserved_usd = get_max_reserved_usd(market_info, is_long);
    
    let open_interest_usd = get_open_interest_usd(market_info, is_long);
    let max_open_interest_usd = get_max_open_interest_usd(market_info, is_long);
    
    if (max_reserved_usd - reserved_usd) < (max_open_interest_usd - open_interest_usd) {
        (reserved_usd, max_reserved_usd)
    } else {
        (open_interest_usd, max_open_interest_usd)
    }
}

pub fn format_amount_human(
    amount: Option<U256>,
    token_decimals: u32,
    show_dollar: bool,
) -> String {
    let n: f64 = format_amount(amount, token_decimals, None, None, None)
        .parse()
        .unwrap_or(0.0);
    
    let is_negative = n < 0.0;
    let abs_n = n.abs();
    let sign = if show_dollar { "$" } else { "" };
    
    if abs_n >= 1_000_000.0 {
        format!("{}{}{:.1}M", if is_negative { "-" } else { "" }, sign, abs_n / 1_000_000.0)
    } else if abs_n >= 1_000.0 {
        format!("{}{}{:.1}K", if is_negative { "-" } else { "" }, sign, abs_n / 1_000.0)
    } else {
        format!("{}{}{:.1}", if is_negative { "-" } else { "" }, sign, abs_n)
    }
}