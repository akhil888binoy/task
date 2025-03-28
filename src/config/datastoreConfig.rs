

use alloy_primitives::{keccak256, Bytes, B256};
use alloy_sol_types::{sol, SolValue};
use crate::utils::hash::{hash_data, hash_string};
use once_cell::sync::Lazy;
use alloy::{primitives::{Address}};



pub static POSITION_IMPACT_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("POSITION_IMPACT_FACTOR")
});
pub static MAX_POSITION_IMPACT_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_POSITION_IMPACT_FACTOR")
});

pub static POSITION_IMPACT_EXPONENT_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("POSITION_IMPACT_EXPONENT_FACTOR")
});

pub static POSITION_FEE_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("POSITION_FEE_FACTOR")
});

pub static SWAP_IMPACT_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("SWAP_IMPACT_FACTOR")
});

pub static SWAP_IMPACT_EXPONENT_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("SWAP_IMPACT_EXPONENT_FACTOR")
});

pub static SWAP_FEE_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("SWAP_FEE_FACTOR")
});

pub static FEE_RECEIVER_DEPOSIT_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("FEE_RECEIVER_DEPOSIT_FACTOR")
});

pub static BORROWING_FEE_RECEIVER_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("BORROWING_FEE_RECEIVER_FACTOR")
});

pub static FEE_RECEIVER_WITHDRAWAL_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("FEE_RECEIVER_WITHDRAWAL_FACTOR")
});

pub static FEE_RECEIVER_SWAP_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("FEE_RECEIVER_SWAP_FACTOR")
});

pub static FEE_RECEIVER_POSITION_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("FEE_RECEIVER_POSITION_FACTOR")
});

pub static OPEN_INTEREST_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("OPEN_INTEREST")
});

pub static OPEN_INTEREST_IN_TOKENS_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("OPEN_INTEREST_IN_TOKENS")
});

pub static POOL_AMOUNT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("POOL_AMOUNT")
});

pub static MAX_POOL_AMOUNT_FOR_DEPOSIT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_POOL_AMOUNT_FOR_DEPOSIT")
});

pub static MAX_POOL_AMOUNT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_POOL_AMOUNT")
});

pub static RESERVE_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("RESERVE_FACTOR")
});

pub static OPEN_INTEREST_RESERVE_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("OPEN_INTEREST_RESERVE_FACTOR")
});

pub static MAX_OPEN_INTEREST_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_OPEN_INTEREST")
});

pub static NONCE_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("NONCE")
});

pub static BORROWING_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("BORROWING_FACTOR")
});

pub static BORROWING_EXPONENT_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("BORROWING_EXPONENT_FACTOR")
});

pub static CUMULATIVE_BORROWING_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("CUMULATIVE_BORROWING_FACTOR")
});

pub static TOTAL_BORROWING_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("TOTAL_BORROWING")
});

pub static FUNDING_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("FUNDING_FACTOR")
});

pub static FUNDING_EXPONENT_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("FUNDING_EXPONENT_FACTOR")
});

pub static FUNDING_INCREASE_FACTOR_PER_SECOND: Lazy<B256> = Lazy::new(|| {
    hash_string("FUNDING_INCREASE_FACTOR_PER_SECOND")
});

pub static FUNDING_DECREASE_FACTOR_PER_SECOND: Lazy<B256> = Lazy::new(|| {
    hash_string("FUNDING_DECREASE_FACTOR_PER_SECOND")
});

pub static MIN_FUNDING_FACTOR_PER_SECOND: Lazy<B256> = Lazy::new(|| {
    hash_string("MIN_FUNDING_FACTOR_PER_SECOND")
});

pub static MAX_FUNDING_FACTOR_PER_SECOND: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_FUNDING_FACTOR_PER_SECOND")
});

pub static THRESHOLD_FOR_STABLE_FUNDING: Lazy<B256> = Lazy::new(|| {
    hash_string("THRESHOLD_FOR_STABLE_FUNDING")
});

pub static THRESHOLD_FOR_DECREASE_FUNDING: Lazy<B256> = Lazy::new(|| {
    hash_string("THRESHOLD_FOR_DECREASE_FUNDING")
});

pub static MAX_PNL_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_PNL_FACTOR")
});

pub static MAX_PNL_FACTOR_FOR_WITHDRAWALS_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_PNL_FACTOR_FOR_WITHDRAWALS")
});

pub static MAX_PNL_FACTOR_FOR_DEPOSITS_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_PNL_FACTOR_FOR_DEPOSITS")
});

pub static MAX_PNL_FACTOR_FOR_TRADERS_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_PNL_FACTOR_FOR_TRADERS")
});

pub static MAX_POSITION_IMPACT_FACTOR_FOR_LIQUIDATIONS_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_POSITION_IMPACT_FACTOR_FOR_LIQUIDATIONS")
});

pub static POSITION_IMPACT_POOL_AMOUNT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("POSITION_IMPACT_POOL_AMOUNT")
});

pub static MIN_POSITION_IMPACT_POOL_AMOUNT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MIN_POSITION_IMPACT_POOL_AMOUNT")
});

pub static POSITION_IMPACT_POOL_DISTRIBUTION_RATE_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("POSITION_IMPACT_POOL_DISTRIBUTION_RATE")
});

pub static SWAP_IMPACT_POOL_AMOUNT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("SWAP_IMPACT_POOL_AMOUNT")
});

pub static MIN_COLLATERAL_USD_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MIN_COLLATERAL_USD")
});

pub static MIN_COLLATERAL_FACTOR_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MIN_COLLATERAL_FACTOR")
});

pub static MIN_COLLATERAL_FACTOR_FOR_OPEN_INTEREST_MULTIPLIER_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MIN_COLLATERAL_FACTOR_FOR_OPEN_INTEREST_MULTIPLIER")
});

pub static MIN_POSITION_SIZE_USD_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MIN_POSITION_SIZE_USD")
});

pub static MAX_LEVERAGE_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_LEVERAGE")
});

pub static DEPOSIT_GAS_LIMIT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("DEPOSIT_GAS_LIMIT")
});

pub static WITHDRAWAL_GAS_LIMIT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("WITHDRAWAL_GAS_LIMIT")
});

pub static INCREASE_ORDER_GAS_LIMIT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("INCREASE_ORDER_GAS_LIMIT")
});

pub static DECREASE_ORDER_GAS_LIMIT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("DECREASE_ORDER_GAS_LIMIT")
});

pub static SWAP_ORDER_GAS_LIMIT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("SWAP_ORDER_GAS_LIMIT")
});

pub static SINGLE_SWAP_GAS_LIMIT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("SINGLE_SWAP_GAS_LIMIT")
});

pub static TOKEN_TRANSFER_GAS_LIMIT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("TOKEN_TRANSFER_GAS_LIMIT")
});

pub static NATIVE_TOKEN_TRANSFER_GAS_LIMIT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("NATIVE_TOKEN_TRANSFER_GAS_LIMIT")
});

pub static ESTIMATED_GAS_FEE_BASE_AMOUNT: Lazy<B256> = Lazy::new(|| {
    hash_string("ESTIMATED_GAS_FEE_BASE_AMOUNT")
});

pub static ESTIMATED_GAS_FEE_MULTIPLIER_FACTOR: Lazy<B256> = Lazy::new(|| {
    hash_string("ESTIMATED_GAS_FEE_MULTIPLIER_FACTOR")
});

pub static MARKET_LIST_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("MARKET_LIST")
});

pub static SINGLE_MARKET_LIST: Lazy<B256> = Lazy::new(|| {
    hash_string("SINGLE_MARKET_LIST")
});

pub static DYNAMIC_MARKET_LIST: Lazy<B256> = Lazy::new(|| {
    hash_string("DYNAMIC_MARKET_LIST")
});

pub static POSITION_LIST_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("POSITION_LIST")
});

pub static ACCOUNT_POSITION_LIST_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("ACCOUNT_POSITION_LIST")
});

pub static ORDER_LIST_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("ORDER_LIST")
});

pub static ACCOUNT_ORDER_LIST_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("ACCOUNT_ORDER_LIST")
});

pub static CLAIMABLE_FUNDING_AMOUNT: Lazy<B256> = Lazy::new(|| {
    hash_string("CLAIMABLE_FUNDING_AMOUNT")
});

pub static VIRTUAL_TOKEN_ID_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("VIRTUAL_TOKEN_ID")
});

pub static VIRTUAL_MARKET_ID_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("VIRTUAL_MARKET_ID")
});

pub static VIRTUAL_INVENTORY_FOR_POSITIONS_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("VIRTUAL_INVENTORY_FOR_POSITIONS")
});

pub static VIRTUAL_INVENTORY_FOR_SWAPS_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("VIRTUAL_INVENTORY_FOR_SWAPS")
});

pub static POOL_AMOUNT_ADJUSTMENT_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("POOL_AMOUNT_ADJUSTMENT")
});

pub static AFFILIATE_REWARD_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("AFFILIATE_REWARD")
});

pub static IS_MARKET_DISABLED_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("IS_MARKET_DISABLED")
});

pub static UI_FEE_FACTOR: Lazy<B256> = Lazy::new(|| {
    hash_string("UI_FEE_FACTOR")
});

pub static SUBACCOUNT_LIST_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("SUBACCOUNT_LIST")
});

pub static MAX_ALLOWED_SUBACCOUNT_ACTION_COUNT: Lazy<B256> = Lazy::new(|| {
    hash_string("MAX_ALLOWED_SUBACCOUNT_ACTION_COUNT")
});

pub static SUBACCOUNT_ACTION_COUNT: Lazy<B256> = Lazy::new(|| {
    hash_string("SUBACCOUNT_ACTION_COUNT")
});

pub static SUBACCOUNT_ORDER_ACTION: Lazy<B256> = Lazy::new(|| {
    hash_string("SUBACCOUNT_ORDER_ACTION")
});

pub static SUBACCOUNT_AUTO_TOP_UP_AMOUNT: Lazy<B256> = Lazy::new(|| {
    hash_string("SUBACCOUNT_AUTO_TOP_UP_AMOUNT")
});

pub static INDEX_TOKENS_LIST_KEY: Lazy<B256> = Lazy::new(|| {
    hash_string("INDEX_TOKENS_LIST")
});


pub fn position_impact_factor_key(market_addr: Address, is_positive: bool) -> B256 {
    
    let factor_key = *POSITION_IMPACT_FACTOR_KEY;
    
    let params = (factor_key, market_addr, is_positive);
    
    hash_data(&params)
}

//START FROM HERE /////////////////////////

pub fn position_impact_factor_key_for_dynamic_market(market: &str, index_token: &str, is_positive: bool) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let factor_key = *POSITION_IMPACT_FACTOR_KEY;
    
    let params = (factor_key, market_addr, index_token_addr, is_positive);
    hash_data(&params)
}

pub fn position_impact_exponent_factor_key(market_addr: Address) -> B256 {
    let factor_key = *POSITION_IMPACT_EXPONENT_FACTOR_KEY;
    
    let params = (factor_key, market_addr);
    hash_data(&params)
}

pub fn position_impact_exponent_factor_key_for_dynamic_market(market: &str, index_token: &str) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let factor_key = *POSITION_IMPACT_EXPONENT_FACTOR_KEY;
    
    let params = (factor_key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn max_position_impact_factor_key(market_addr: Address, is_positive: bool) -> B256 {
    let factor_key = *MAX_POSITION_IMPACT_FACTOR_KEY;
    
    let params = (factor_key, market_addr, is_positive);
    hash_data(&params)
}

pub fn max_position_impact_factor_key_for_dynamic_market(market: &str, index_token: &str, is_positive: bool) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let factor_key = *MAX_POSITION_IMPACT_FACTOR_KEY;
    
    let params = (factor_key, market_addr, index_token_addr, is_positive);
    hash_data(&params)
}

pub fn position_fee_factor_key(market_addr: Address, for_positive_impact: bool) -> B256 {
    let factor_key = *POSITION_FEE_FACTOR_KEY;
    
    let params = (factor_key, market_addr, for_positive_impact);
    hash_data(&params)
}

pub fn position_fee_factor_key_for_dynamic_market(market: &str, index_token: &str, for_positive_impact: bool) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let factor_key = *POSITION_FEE_FACTOR_KEY;
    
    let params = (factor_key, market_addr, index_token_addr, for_positive_impact);
    hash_data(&params)
}

pub fn swap_impact_factor_key(market_addr: Address, is_positive: bool) -> B256 {
    let factor_key = *SWAP_IMPACT_FACTOR_KEY;
    
    let params = (factor_key, market_addr, is_positive);
    hash_data(&params)
}

pub fn swap_impact_exponent_factor_key(market_addr: Address) -> B256 {
    let factor_key = *SWAP_IMPACT_EXPONENT_FACTOR_KEY;
    
    let params = (factor_key, market_addr);
    hash_data(&params)
}

pub fn swap_fee_factor_key(market_addr: Address, for_positive_impact: bool) -> B256 {
    let factor_key = *SWAP_FEE_FACTOR_KEY;
    
    let params = (factor_key, market_addr, for_positive_impact);
    hash_data(&params)
}

pub fn open_interest_in_tokens_key(market_addr: Address, collateral_token_addr: Address, is_long: bool) -> B256 {
    let key = *OPEN_INTEREST_IN_TOKENS_KEY;
    
    let params = (key, market_addr, collateral_token_addr, is_long);
    hash_data(&params)
}

pub fn open_interest_in_tokens_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    collateral_token: &str, 
    is_long: bool
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let collateral_token_addr: Address = collateral_token.parse().expect("Invalid collateral token address");
    let key = *OPEN_INTEREST_IN_TOKENS_KEY;
    
    let params = (key, market_addr, index_token_addr, collateral_token_addr, is_long);
    hash_data(&params)
}

pub fn pool_amount_key(market_addr: Address, token_addr: Address) -> B256 {
    let key = *POOL_AMOUNT_KEY;
    let params = (key, market_addr, token_addr);
    hash_data(&params)
}

pub fn pool_amount_key_for_dynamic_market(market: &str, index: &str, token: &str) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_addr: Address = index.parse().expect("Invalid index address");
    let token_addr: Address = token.parse().expect("Invalid token address");
    let key = *POOL_AMOUNT_KEY;
    
    let params = (key, market_addr, index_addr, token_addr);
    hash_data(&params)
}

pub fn reserve_factor_key(market_addr: Address, is_long: bool) -> B256 {
    let key = *RESERVE_FACTOR_KEY;
    
    let params = (key, market_addr, is_long);
    hash_data(&params)
}

pub fn reserve_factor_key_for_dynamic_market(market: &str, index_token: &str, is_long: bool) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *RESERVE_FACTOR_KEY;
    
    let params = (key, market_addr, index_token_addr, is_long);
    hash_data(&params)
}

pub fn open_interest_reserve_factor_key(market_addr: Address, is_long: bool) -> B256 {
    let key = *OPEN_INTEREST_RESERVE_FACTOR_KEY;
    
    let params = (key, market_addr, is_long);
    hash_data(&params)
}

pub fn open_interest_reserve_factor_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    is_long: bool
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *OPEN_INTEREST_RESERVE_FACTOR_KEY;
    
    let params = (key, market_addr, index_token_addr, is_long);
    hash_data(&params)
}

pub fn max_open_interest_key(market_addr: Address, is_long: bool) -> B256 {
    let key = *MAX_OPEN_INTEREST_KEY;
    
    let params = (key, market_addr, is_long);
    hash_data(&params)
}

pub fn max_open_interest_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    long_token_address: &str, 
    is_long: bool
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let long_token_addr: Address = long_token_address.parse().expect("Invalid long token address");
    let key = *MAX_OPEN_INTEREST_KEY;
    
    let params = (key, market_addr, index_token_addr, long_token_addr, is_long);
    hash_data(&params)
}

pub fn borrowing_factor_key(market_addr: Address, is_long: bool) -> B256 {
    let key = *BORROWING_FACTOR_KEY;
    
    let params = (key, market_addr, is_long);
    hash_data(&params)
}

pub fn borrowing_factor_key_for_dynamic_market(market: &str, index_token: &str, is_long: bool) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *BORROWING_FACTOR_KEY;
    
    let params = (key, market_addr, index_token_addr, is_long);
    hash_data(&params)
}

pub fn borrowing_exponent_factor_key(market_addr: Address, is_long: bool) -> B256 {
    let key = *BORROWING_EXPONENT_FACTOR_KEY;
    
    let params = (key, market_addr, is_long);
    hash_data(&params)
}

pub fn borrowing_exponent_factor_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    is_long: bool
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *BORROWING_EXPONENT_FACTOR_KEY;
    
    let params = (key, market_addr, index_token_addr, is_long);
    hash_data(&params)
}

pub fn cumulative_borrowing_factor_key(market: &str, is_long: bool) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let key = *CUMULATIVE_BORROWING_FACTOR_KEY;
    
    let params = (key, market_addr, is_long);
    hash_data(&params)
}

pub fn cumulative_borrowing_factor_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    is_long: bool
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *CUMULATIVE_BORROWING_FACTOR_KEY;
    
    let params = (key, market_addr, index_token_addr, is_long);
    hash_data(&params)
}

pub fn total_borrowing_key(market: &str, is_long: bool) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let key = *TOTAL_BORROWING_KEY;
    
    let params = (key, market_addr, is_long);
    hash_data(&params)
}

pub fn total_borrowing_key_for_dynamic_market(market: &str, index_token: &str, is_long: bool) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *TOTAL_BORROWING_KEY;
    
    let params = (key, market_addr, index_token_addr, is_long);
    hash_data(&params)
}

pub fn funding_factor_key(market_addr: Address) -> B256 {
    let key = *FUNDING_FACTOR_KEY;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn funding_factor_key_for_dynamic_market(market: &str, index_token: &str) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *FUNDING_FACTOR_KEY;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn funding_exponent_factor_key(market_addr: Address) -> B256 {
    let key = *FUNDING_EXPONENT_FACTOR_KEY;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn funding_exponent_factor_key_for_dynamic_market(market: &str, index_token: &str) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *FUNDING_EXPONENT_FACTOR_KEY;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn funding_increase_factor_per_second_key(market_addr: Address) -> B256 {
    let key = *FUNDING_INCREASE_FACTOR_PER_SECOND;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn funding_increase_factor_per_second_for_dynamic_market_key(
    market: &str, 
    index_token: &str
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *FUNDING_INCREASE_FACTOR_PER_SECOND;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn funding_decrease_factor_per_second_key(market_addr: Address) -> B256 {
    let key = *FUNDING_DECREASE_FACTOR_PER_SECOND;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn funding_decrease_factor_per_second_for_dynamic_market_key(
    market: &str, 
    index_token: &str
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *FUNDING_DECREASE_FACTOR_PER_SECOND;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn min_funding_factor_per_second_key(market_addr: Address) -> B256 {
    let key = *MIN_FUNDING_FACTOR_PER_SECOND;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn min_funding_factor_per_second_for_dynamic_market_key(
    market: &str, 
    index_token: &str
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *MIN_FUNDING_FACTOR_PER_SECOND;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn max_funding_factor_per_second_key(market_addr: Address) -> B256 {
    let key = *MAX_FUNDING_FACTOR_PER_SECOND;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn max_funding_factor_per_second_for_dynamic_market_key(
    market: &str, 
    index_token: &str
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *MAX_FUNDING_FACTOR_PER_SECOND;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn threshold_for_stable_funding_key(market_addr: Address) -> B256 {
    let key = *THRESHOLD_FOR_STABLE_FUNDING;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn threshold_for_stable_funding_key_for_dynamic_market(
    market: &str, 
    index_token: &str
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *THRESHOLD_FOR_STABLE_FUNDING;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn threshold_for_decrease_funding_key(market_addr: Address) -> B256 {
    let key = *THRESHOLD_FOR_DECREASE_FUNDING;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn threshold_for_decrease_funding_key_dynamic_market(
    market: &str, 
    index_token: &str
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *THRESHOLD_FOR_DECREASE_FUNDING;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn max_pnl_factor_key(pnl_factor_type_bytes: B256, market_addr: Address, is_long: bool) -> B256 {
    let key = *MAX_PNL_FACTOR_KEY;
    
    let params = (key, pnl_factor_type_bytes, market_addr, is_long);
    hash_data(&params)
}

pub fn max_pnl_factor_key_for_dynamic_market(
    pnl_factor_type: &str, 
    market: &str, 
    index_token: &str, 
    is_long: bool
) -> B256 {
    let pnl_factor_type_bytes: B256 = pnl_factor_type.parse().expect("Invalid pnl factor type");
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let key = *MAX_PNL_FACTOR_KEY;
    
    let params = (key, pnl_factor_type_bytes, market_addr, index_token_addr, is_long);
    hash_data(&params)
}

pub fn position_impact_pool_amount_key(market_addr: Address) -> B256 {
    let key = *POSITION_IMPACT_POOL_AMOUNT_KEY;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn position_impact_pool_amount_key_for_dynamic_market(market_addr: Address, index_token_addr: Address) -> B256 {
    let key = *POSITION_IMPACT_POOL_AMOUNT_KEY;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn min_position_impact_pool_amount_key(market_addr: Address) -> B256 {
    let key = *MIN_POSITION_IMPACT_POOL_AMOUNT_KEY;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn min_position_impact_pool_amount_key_for_dynamic_market(market_addr: Address, index_token_addr: &Address) -> B256 {
    let key = *MIN_POSITION_IMPACT_POOL_AMOUNT_KEY;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn position_impact_pool_distribution_rate_key(market_addr: Address) -> B256 {
    let key = *POSITION_IMPACT_POOL_DISTRIBUTION_RATE_KEY;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn max_position_impact_factor_for_liquidations_key(market_addr: Address) -> B256 {
    let key = *MAX_POSITION_IMPACT_FACTOR_FOR_LIQUIDATIONS_KEY;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn max_position_impact_factor_key_for_liquidations_for_dynamic_market(
    market_addr: Address, 
    index_token_addr: Address
) -> B256 {
    let key = *MAX_POSITION_IMPACT_FACTOR_FOR_LIQUIDATIONS_KEY;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn swap_impact_pool_amount_key(market_addr: Address, token_addr: Address) -> B256 {
    let key = *SWAP_IMPACT_POOL_AMOUNT_KEY;
    
    let params = (key, market_addr, token_addr);
    hash_data(&params)
}

pub fn min_collateral_factor_key(market_addr: Address) -> B256 {
    let key = *MIN_COLLATERAL_FACTOR_KEY;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn min_collateral_factor_key_for_dynamic_market(market_addr: Address, index_token_addr: Address) -> B256 {
    let key = *MIN_COLLATERAL_FACTOR_KEY;
    
    let params = (key, market_addr, index_token_addr);
    hash_data(&params)
}

pub fn min_collateral_factor_for_open_interest(market_addr: Address, is_long: bool) -> B256 {
    let key = *MIN_COLLATERAL_FACTOR_FOR_OPEN_INTEREST_MULTIPLIER_KEY;
    
    let params = (key, market_addr, is_long);
    hash_data(&params)
}

pub fn min_collateral_factor_for_open_interest_key_for_dynamic_market(
    market_addr: Address, 
    index_token_addr: Address, 
    is_long: bool
) -> B256 {
    let key = *MIN_COLLATERAL_FACTOR_FOR_OPEN_INTEREST_MULTIPLIER_KEY;
    
    let params = (key, market_addr, index_token_addr, is_long);
    hash_data(&params)
}

pub fn claimable_funding_amount_key(market_addr: Address, token_addr: Address, account_addr: Address) -> B256 {
    let key = *CLAIMABLE_FUNDING_AMOUNT;
    
    let params = (key, market_addr, token_addr, account_addr);
    hash_data(&params)
}

pub fn claimable_funding_amount_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    token: &str, 
    account: &str
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let token_addr: Address = token.parse().expect("Invalid token address");
    let account_addr: Address = account.parse().expect("Invalid account address");
    let key = *CLAIMABLE_FUNDING_AMOUNT;
    
    let params = (key, market_addr, index_token_addr, token_addr, account_addr);
    hash_data(&params)
}

pub fn virtual_token_id_key(token_addr: Address) -> B256 {
    let key = *VIRTUAL_TOKEN_ID_KEY;
    
    let params = (key, token_addr);
    hash_data(&params)
}

pub fn virtual_token_id_key_for_dynamic_market(
    token: &str, 
    index_token: &str, 
    collateral_token: &str
) -> B256 {
    let token_addr: Address = token.parse().expect("Invalid token address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let collateral_token_addr: Address = collateral_token.parse().expect("Invalid collateral token address");
    let key = *VIRTUAL_TOKEN_ID_KEY;
    
    let params = (key, token_addr, index_token_addr, collateral_token_addr);
    hash_data(&params)
}

pub fn virtual_market_id_key(market_addr: Address) -> B256 {
    let key = *VIRTUAL_MARKET_ID_KEY;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn virtual_inventory_for_swaps_key(virtual_market_id: &str, token: &str) -> B256 {
    let virtual_market_id_bytes: B256 = virtual_market_id.parse().expect("Invalid virtual market id");
    let token_addr: Address = token.parse().expect("Invalid token address");
    let key = *VIRTUAL_INVENTORY_FOR_SWAPS_KEY;
    
    let params = (key, virtual_market_id_bytes, token_addr);
    hash_data(&params)
}

pub fn virtual_inventory_for_positions_key(virtual_token_id: &str) -> B256 {
    let virtual_token_id_bytes: B256 = virtual_token_id.parse().expect("Invalid virtual token id");
    let key = *VIRTUAL_INVENTORY_FOR_POSITIONS_KEY;
    
    let params = (key, virtual_token_id_bytes);
    hash_data(&params)
}

pub fn pool_amount_adjustment_key(market_addr: Address, token_addr: Address) -> B256 {
    let key = *POOL_AMOUNT_ADJUSTMENT_KEY;
    
    let params = (key, market_addr, token_addr);
    hash_data(&params)
}

pub fn affiliate_reward_key(market: &str, token: &str, account: &str) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let token_addr: Address = token.parse().expect("Invalid token address");
    let account_addr: Address = account.parse().expect("Invalid account address");
    let key = *AFFILIATE_REWARD_KEY;
    
    let params = (key, market_addr, token_addr, account_addr);
    hash_data(&params)
}

pub fn is_market_disabled_key(market_addr:Address) -> B256 {
    let key = *IS_MARKET_DISABLED_KEY;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn max_pool_amount_for_deposit_key(market_addr: Address, token_addr: Address) -> B256 {
    let key = *MAX_POOL_AMOUNT_FOR_DEPOSIT_KEY;
    let params = (key, market_addr, token_addr);
    hash_data(&params)
}

pub fn max_pool_amount_for_deposit_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    token: &str
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let token_addr: Address = token.parse().expect("Invalid token address");
    let key = *MAX_POOL_AMOUNT_FOR_DEPOSIT_KEY;
    
    let params = (key, market_addr, index_token_addr, token_addr);
    hash_data(&params)
}

pub fn max_pool_amount_key(market_addr: Address, token_addr: Address) -> B256 {
    let key = *MAX_POOL_AMOUNT_KEY;
    
    let params = (key, market_addr, token_addr);
    hash_data(&params)
}

pub fn max_pool_amount_key_for_dynamic_market(market: &str, index_token: &str, token: &str) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let token_addr: Address = token.parse().expect("Invalid token address");
    let key = *MAX_POOL_AMOUNT_KEY;
    
    let params = (key, market_addr, index_token_addr, token_addr);
    hash_data(&params)
}

pub fn ui_fee_factor_key(address: &str) -> B256 {
    let address_addr: Address = address.parse().expect("Invalid address");
    let key = *UI_FEE_FACTOR;
    
    let params = (key, address_addr);
    hash_data(&params)
}

pub fn subaccount_list_key(account: &str) -> B256 {
    let account_addr: Address = account.parse().expect("Invalid account address");
    let key = *SUBACCOUNT_LIST_KEY;
    
    let params = (key, account_addr);
    hash_data(&params)
}

pub fn max_allowed_subaccount_action_count_key(
    account: &str, 
    subaccount: &str, 
    action_type: &str
) -> B256 {
    let account_addr: Address = account.parse().expect("Invalid account address");
    let subaccount_addr: Address = subaccount.parse().expect("Invalid subaccount address");
    let action_type_bytes: B256 = action_type.parse().expect("Invalid action type");
    let key = *MAX_ALLOWED_SUBACCOUNT_ACTION_COUNT;
    
    let params = (key, account_addr, subaccount_addr, action_type_bytes);
    hash_data(&params)
}

pub fn subaccount_action_count_key(
    account: &str, 
    subaccount: &str, 
    action_type: &str
) -> B256 {
    let account_addr: Address = account.parse().expect("Invalid account address");
    let subaccount_addr: Address = subaccount.parse().expect("Invalid subaccount address");
    let action_type_bytes: B256 = action_type.parse().expect("Invalid action type");
    let key = *SUBACCOUNT_ACTION_COUNT;
    
    let params = (key, account_addr, subaccount_addr, action_type_bytes);
    hash_data(&params)
}

pub fn subaccount_auto_top_up_amount_key(account: &str, subaccount: &str) -> B256 {
    let account_addr: Address = account.parse().expect("Invalid account address");
    let subaccount_addr: Address = subaccount.parse().expect("Invalid subaccount address");
    let key = *SUBACCOUNT_AUTO_TOP_UP_AMOUNT;
    
    let params = (key, account_addr, subaccount_addr);
    hash_data(&params)
}

pub fn index_tokens_list_key(market: &str) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let key = *INDEX_TOKENS_LIST_KEY;
    
    let params = (key, market_addr);
    hash_data(&params)
}

pub fn open_interest_key(market_addr: Address, collateral_token_addr: Address, is_long: bool) -> B256 {
    let key = *OPEN_INTEREST_KEY;
    
    let params = (key, market_addr, collateral_token_addr, is_long);
    hash_data(&params)
}

pub fn open_interest_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    collateral_token: &str, 
    is_long: bool
) -> B256 {
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let collateral_token_addr: Address = collateral_token.parse().expect("Invalid collateral token address");
    let key = *OPEN_INTEREST_KEY;
    
    let params = (key, market_addr, index_token_addr, collateral_token_addr, is_long);
    hash_data(&params)
}

pub fn hashed_position_key(account: &str, market: &str, collateral_token: &str, is_long: bool) -> B256 {
    let account_addr: Address = account.parse().expect("Invalid account address");
    let market_addr: Address = market.parse().expect("Invalid market address");
    let collateral_token_addr: Address = collateral_token.parse().expect("Invalid collateral token address");
    
    let params = (account_addr, market_addr, collateral_token_addr, is_long);
    hash_data(&params)
}

pub fn hashed_position_key_for_dynamic_market(
    account: &str, 
    market: &str, 
    index_token: &str, 
    collateral_token: &str, 
    is_long: bool
) -> B256 {
    let account_addr: Address = account.parse().expect("Invalid account address");
    let market_addr: Address = market.parse().expect("Invalid market address");
    let index_token_addr: Address = index_token.parse().expect("Invalid index token address");
    let collateral_token_addr: Address = collateral_token.parse().expect("Invalid collateral token address");
    
    let params = (account_addr, market_addr, index_token_addr, collateral_token_addr, is_long);
    hash_data(&params)
}

pub fn order_key(data_store_address: &str, nonce: u64) -> B256 {
    let data_store_addr: Address = data_store_address.parse().expect("Invalid data store address");
    
    let params = (data_store_addr, nonce);
    hash_data(&params)
}

pub fn deposit_gas_limit_key(single_token: bool) -> B256 {
    let key = *DEPOSIT_GAS_LIMIT_KEY;
    
    let params = (key, single_token);
    hash_data(&params)
}

pub fn withdrawal_gas_limit_key() -> B256 {
    let key = *WITHDRAWAL_GAS_LIMIT_KEY;
    
    hash_data(&key)
}

pub fn single_swap_gas_limit_key() -> B256 {
    *SINGLE_SWAP_GAS_LIMIT_KEY
}

pub fn increase_order_gas_limit_key() -> B256 {
    *INCREASE_ORDER_GAS_LIMIT_KEY
}

pub fn decrease_order_gas_limit_key() -> B256 {
    *DECREASE_ORDER_GAS_LIMIT_KEY
}

pub fn swap_order_gas_limit_key() -> B256 {
    *SWAP_ORDER_GAS_LIMIT_KEY
}

pub fn account_order_list_key(account: &str) -> B256 {
    let account_addr: Address = account.parse().expect("Invalid account address");
    let key = *ACCOUNT_ORDER_LIST_KEY;
    
    let params = (key, account_addr);
    hash_data(&params)
}

pub fn account_position_list_key(account: &str) -> B256 {
    let account_addr: Address = account.parse().expect("Invalid account address");
    let key = *ACCOUNT_POSITION_LIST_KEY;
    
    let params = (key, account_addr);
    hash_data(&params)
}