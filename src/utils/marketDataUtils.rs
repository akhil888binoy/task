use alloy::primitives::{U256, Address};
use std::cmp::{min, max};
use once_cell::sync::Lazy;
use crate::types::types::{MarketInfo, TokenData, TokenPrices};
use crate::utils::hash::expand_decimals;

pub static PRECISION: Lazy<U256> = Lazy::new(|| expand_decimals(U256::from(1), 30));
pub const USD_DECIMALS: u32 = 30;

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
    let capacity_amount = max_pool_amount.saturating_sub(pool_amount);
    
    max(capacity_amount, U256::ZERO)
}

pub fn get_deposit_collateral_capacity_usd(market_info: &MarketInfo, is_long: bool) -> U256 {
    let pool_usd = get_pool_usd_without_pnl(market_info, is_long, PriceType::Mid);
    let max_pool_usd = get_max_pool_usd_for_deposit(market_info, is_long);
    let capacity_usd = max_pool_usd.saturating_sub(pool_usd);
    
    max(capacity_usd, U256::ZERO)
}

#[derive(Clone, Copy)]
pub enum PriceType {
    Min,
    Max,
    Mid,
}

pub fn get_mid_price(prices: &TokenPrices) -> U256 {
    (prices.min_price + prices.max_price) / U256::from(2)
}

pub fn convert_to_usd(
    token_amount: Option<U256>,
    token_decimals: Option<u32>,
    price: Option<U256>,
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
    convert_to_usd(
        Some(max_pool_amount),
        Some(token.token.decimals),
        Some(get_mid_price(&token.prices))
    ).unwrap_or(U256::ZERO)
}

pub fn get_pool_usd_without_pnl(
    market_info: &MarketInfo,
    is_long: bool,
    price_type: PriceType,
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
        PriceType::Min => token.prices.min_price,
        PriceType::Max => token.prices.max_price,
        PriceType::Mid => get_mid_price(&token.prices),
    };
    
    convert_to_usd(Some(pool_amount), Some(token.token.decimals), Some(price))
        .unwrap_or(U256::ZERO)
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

pub fn usd_to_market_token_amount(
    market_info: &MarketInfo,
    market_token: &TokenData,
    usd_value: U256,
) -> U256 {
    let supply = market_token.total_supply.unwrap_or(U256::ZERO);
    let pool_value = market_info.pool_value_max;
    
    // If supply and poolValue are zero, use 1 USD as token price
    if supply.is_zero() && pool_value.is_zero() {
        return convert_to_token_amount(
            Some(usd_value),
            Some(market_token.token.decimals),
            Some(expand_decimals(U256::from(1), USD_DECIMALS))
        ).unwrap_or(U256::ZERO);
    }
    
    // If supply is zero but poolValue exists
    if supply.is_zero() && pool_value > U256::ZERO {
        return convert_to_token_amount(
            Some(usd_value + pool_value),
            Some(market_token.token.decimals),
            Some(expand_decimals(U256::from(1), USD_DECIMALS))
        ).unwrap_or(U256::ZERO);
    }
    
    if pool_value.is_zero() {
        return U256::ZERO;
    }
    
    (supply * usd_value) / pool_value
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

#[derive(Debug)]
pub struct MintableMarketTokens {
    pub mintable_amount: U256,
    pub mintable_usd: U256,
    pub long_deposit_capacity_usd: U256,
    pub short_deposit_capacity_usd: U256,
    pub long_deposit_capacity_amount: U256,
    pub short_deposit_capacity_amount: U256,
}

pub fn get_available_usd_liquidity_for_collateral(
    market_info: &MarketInfo,
    is_long: bool,
) -> U256 {
    let pool_usd = get_pool_usd_without_pnl(market_info, is_long, PriceType::Min);
    
    if market_info.market.is_spot_only {
        return pool_usd;
    }
    
    let reserved_usd = get_reserved_usd(market_info, is_long);
    let max_reserve_factor = if is_long { 
        market_info.reserve_factor_long 
    } else { 
        market_info.reserve_factor_short 
    };
    
    if max_reserve_factor.is_zero() {
        return U256::ZERO;
    }
    
    let min_pool_usd = (reserved_usd * *PRECISION) / max_reserve_factor;
    let liquidity = pool_usd.saturating_sub(min_pool_usd);
    
    max(liquidity, U256::ZERO)
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
    
    if long_pool_usd.is_zero() || 
       short_pool_usd.is_zero() || 
       long_collateral_liquidity_usd.is_zero() || 
       short_collateral_liquidity_usd.is_zero() {
        return SellableMarketToken {
            max_long_sellable_usd: U256::ZERO,
            max_short_sellable_usd: U256::ZERO,
            total_usd: U256::ZERO,
            total_amount: U256::ZERO,
            max_long_sellable_amount: U256::ZERO,
            max_short_sellable_amount: U256::ZERO,
        };
    }
    
    let ratio = (long_pool_usd * factor) / short_pool_usd;
    let (max_long_sellable_usd, max_short_sellable_usd) = 
        if (short_collateral_liquidity_usd * ratio) / factor <= long_collateral_liquidity_usd {
            (
                (short_collateral_liquidity_usd * ratio) / factor,
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

#[derive(Debug)]
pub struct SellableMarketToken {
    pub max_long_sellable_usd: U256,
    pub max_short_sellable_usd: U256,
    pub max_long_sellable_amount: U256,
    pub max_short_sellable_amount: U256,
    pub total_usd: U256,
    pub total_amount: U256,
}

pub fn get_max_reserved_usd(market_info: &MarketInfo, is_long: bool) -> U256 {
    let pool_usd = get_pool_usd_without_pnl(market_info, is_long, PriceType::Min);
    
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
    
    let is_reserve_smaller = max_reserved_usd - reserved_usd < max_open_interest_usd - open_interest_usd;
    
    if is_reserve_smaller {
        (reserved_usd, max_reserved_usd)
    } else {
        (open_interest_usd, max_open_interest_usd)
    }
}