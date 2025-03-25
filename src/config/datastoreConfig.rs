#[path="../utils/hash.rs"]
mod hash;

use alloy_primitives::{keccak256, Bytes, B256};
use alloy_sol_types::{sol, SolValue};
use hash::{hashData, hashString};


pub const POSITION_IMPACT_FACTOR_KEY: B256 = hash_string("POSITION_IMPACT_FACTOR");
pub const MAX_POSITION_IMPACT_FACTOR_KEY: B256 = hash_string("MAX_POSITION_IMPACT_FACTOR");
pub const POSITION_IMPACT_EXPONENT_FACTOR_KEY: B256 = hash_string("POSITION_IMPACT_EXPONENT_FACTOR");
pub const POSITION_FEE_FACTOR_KEY: B256 = hash_string("POSITION_FEE_FACTOR");
pub const SWAP_IMPACT_FACTOR_KEY: B256 = hash_string("SWAP_IMPACT_FACTOR");
pub const SWAP_IMPACT_EXPONENT_FACTOR_KEY: B256 = hash_string("SWAP_IMPACT_EXPONENT_FACTOR");
pub const SWAP_FEE_FACTOR_KEY: B256 = hash_string("SWAP_FEE_FACTOR");
pub const FEE_RECEIVER_DEPOSIT_FACTOR_KEY: B256 = hash_string("FEE_RECEIVER_DEPOSIT_FACTOR");
pub const BORROWING_FEE_RECEIVER_FACTOR_KEY: B256 = hash_string("BORROWING_FEE_RECEIVER_FACTOR");
pub const FEE_RECEIVER_WITHDRAWAL_FACTOR_KEY: B256 = hash_string("FEE_RECEIVER_WITHDRAWAL_FACTOR");
pub const FEE_RECEIVER_SWAP_FACTOR_KEY: B256 = hash_string("FEE_RECEIVER_SWAP_FACTOR");
pub const FEE_RECEIVER_POSITION_FACTOR_KEY: B256 = hash_string("FEE_RECEIVER_POSITION_FACTOR");
pub const OPEN_INTEREST_KEY: B256 = hash_string("OPEN_INTEREST");
pub const OPEN_INTEREST_IN_TOKENS_KEY: B256 = hash_string("OPEN_INTEREST_IN_TOKENS");
pub const POOL_AMOUNT_KEY: B256 = hash_string("POOL_AMOUNT");
pub const MAX_POOL_AMOUNT_FOR_DEPOSIT_KEY: B256 = hash_string("MAX_POOL_AMOUNT_FOR_DEPOSIT");
pub const MAX_POOL_AMOUNT_KEY: B256 = hash_string("MAX_POOL_AMOUNT");
pub const RESERVE_FACTOR_KEY: B256 = hash_string("RESERVE_FACTOR");
pub const OPEN_INTEREST_RESERVE_FACTOR_KEY: B256 = hash_string("OPEN_INTEREST_RESERVE_FACTOR");
pub const MAX_OPEN_INTEREST_KEY: B256 = hash_string("MAX_OPEN_INTEREST");
pub const NONCE_KEY: B256 = hash_string("NONCE");
pub const BORROWING_FACTOR_KEY: B256 = hash_string("BORROWING_FACTOR");
pub const BORROWING_EXPONENT_FACTOR_KEY: B256 = hash_string("BORROWING_EXPONENT_FACTOR");
pub const CUMULATIVE_BORROWING_FACTOR_KEY: B256 = hash_string("CUMULATIVE_BORROWING_FACTOR");
pub const TOTAL_BORROWING_KEY: B256 = hash_string("TOTAL_BORROWING");
pub const FUNDING_FACTOR_KEY: B256 = hash_string("FUNDING_FACTOR");
pub const FUNDING_EXPONENT_FACTOR_KEY: B256 = hash_string("FUNDING_EXPONENT_FACTOR");
pub const FUNDING_INCREASE_FACTOR_PER_SECOND: B256 = hash_string("FUNDING_INCREASE_FACTOR_PER_SECOND");
pub const FUNDING_DECREASE_FACTOR_PER_SECOND: B256 = hash_string("FUNDING_DECREASE_FACTOR_PER_SECOND");
pub const MIN_FUNDING_FACTOR_PER_SECOND: B256 = hash_string("MIN_FUNDING_FACTOR_PER_SECOND");
pub const MAX_FUNDING_FACTOR_PER_SECOND: B256 = hash_string("MAX_FUNDING_FACTOR_PER_SECOND");
pub const THRESHOLD_FOR_STABLE_FUNDING: B256 = hash_string("THRESHOLD_FOR_STABLE_FUNDING");
pub const THRESHOLD_FOR_DECREASE_FUNDING: B256 = hash_string("THRESHOLD_FOR_DECREASE_FUNDING");
pub const MAX_PNL_FACTOR_KEY: B256 = hash_string("MAX_PNL_FACTOR");
pub const MAX_PNL_FACTOR_FOR_WITHDRAWALS_KEY: B256 = hash_string("MAX_PNL_FACTOR_FOR_WITHDRAWALS");
pub const MAX_PNL_FACTOR_FOR_DEPOSITS_KEY: B256 = hash_string("MAX_PNL_FACTOR_FOR_DEPOSITS");
pub const MAX_PNL_FACTOR_FOR_TRADERS_KEY: B256 = hash_string("MAX_PNL_FACTOR_FOR_TRADERS");
pub const MAX_POSITION_IMPACT_FACTOR_FOR_LIQUIDATIONS_KEY: B256 = hash_string("MAX_POSITION_IMPACT_FACTOR_FOR_LIQUIDATIONS");
pub const POSITION_IMPACT_POOL_AMOUNT_KEY: B256 = hash_string("POSITION_IMPACT_POOL_AMOUNT");
pub const MIN_POSITION_IMPACT_POOL_AMOUNT_KEY: B256 = hash_string("MIN_POSITION_IMPACT_POOL_AMOUNT");
pub const POSITION_IMPACT_POOL_DISTRIBUTION_RATE_KEY: B256 = hash_string("POSITION_IMPACT_POOL_DISTRIBUTION_RATE");
pub const SWAP_IMPACT_POOL_AMOUNT_KEY: B256 = hash_string("SWAP_IMPACT_POOL_AMOUNT");
pub const MIN_COLLATERAL_USD_KEY: B256 = hash_string("MIN_COLLATERAL_USD");
pub const MIN_COLLATERAL_FACTOR_KEY: B256 = hash_string("MIN_COLLATERAL_FACTOR");
pub const MIN_COLLATERAL_FACTOR_FOR_OPEN_INTEREST_MULTIPLIER_KEY: B256 = hash_string("MIN_COLLATERAL_FACTOR_FOR_OPEN_INTEREST_MULTIPLIER");
pub const MIN_POSITION_SIZE_USD_KEY: B256 = hash_string("MIN_POSITION_SIZE_USD");
pub const MAX_LEVERAGE_KEY: B256 = hash_string("MAX_LEVERAGE");
pub const DEPOSIT_GAS_LIMIT_KEY: B256 = hash_string("DEPOSIT_GAS_LIMIT");
pub const WITHDRAWAL_GAS_LIMIT_KEY: B256 = hash_string("WITHDRAWAL_GAS_LIMIT");
pub const INCREASE_ORDER_GAS_LIMIT_KEY: B256 = hash_string("INCREASE_ORDER_GAS_LIMIT");
pub const DECREASE_ORDER_GAS_LIMIT_KEY: B256 = hash_string("DECREASE_ORDER_GAS_LIMIT");
pub const SWAP_ORDER_GAS_LIMIT_KEY: B256 = hash_string("SWAP_ORDER_GAS_LIMIT");
pub const SINGLE_SWAP_GAS_LIMIT_KEY: B256 = hash_string("SINGLE_SWAP_GAS_LIMIT");
pub const TOKEN_TRANSFER_GAS_LIMIT_KEY: B256 = hash_string("TOKEN_TRANSFER_GAS_LIMIT");
pub const NATIVE_TOKEN_TRANSFER_GAS_LIMIT_KEY: B256 = hash_string("NATIVE_TOKEN_TRANSFER_GAS_LIMIT");
pub const ESTIMATED_GAS_FEE_BASE_AMOUNT: B256 = hash_string("ESTIMATED_GAS_FEE_BASE_AMOUNT");
pub const ESTIMATED_GAS_FEE_MULTIPLIER_FACTOR: B256 = hash_string("ESTIMATED_GAS_FEE_MULTIPLIER_FACTOR");
pub const MARKET_LIST_KEY: B256 = hash_string("MARKET_LIST");
pub const SINGLE_MARKET_LIST: B256 = hash_string("SINGLE_MARKET_LIST");
pub const DYNAMIC_MARKET_LIST: B256 = hash_string("DYNAMIC_MARKET_LIST");
pub const POSITION_LIST_KEY: B256 = hash_string("POSITION_LIST");
pub const ACCOUNT_POSITION_LIST_KEY: B256 = hash_string("ACCOUNT_POSITION_LIST");
pub const ORDER_LIST_KEY: B256 = hash_string("ORDER_LIST");
pub const ACCOUNT_ORDER_LIST_KEY: B256 = hash_string("ACCOUNT_ORDER_LIST");
pub const CLAIMABLE_FUNDING_AMOUNT: B256 = hash_string("CLAIMABLE_FUNDING_AMOUNT");
pub const VIRTUAL_TOKEN_ID_KEY: B256 = hash_string("VIRTUAL_TOKEN_ID");
pub const VIRTUAL_MARKET_ID_KEY: B256 = hash_string("VIRTUAL_MARKET_ID");
pub const VIRTUAL_INVENTORY_FOR_POSITIONS_KEY: B256 = hash_string("VIRTUAL_INVENTORY_FOR_POSITIONS");
pub const VIRTUAL_INVENTORY_FOR_SWAPS_KEY: B256 = hash_string("VIRTUAL_INVENTORY_FOR_SWAPS");
pub const POOL_AMOUNT_ADJUSTMENT_KEY: B256 = hash_string("POOL_AMOUNT_ADJUSTMENT");
pub const AFFILIATE_REWARD_KEY: B256 = hash_string("AFFILIATE_REWARD");
pub const IS_MARKET_DISABLED_KEY: B256 = hash_string("IS_MARKET_DISABLED");
pub const UI_FEE_FACTOR: B256 = hash_string("UI_FEE_FACTOR");
pub const SUBACCOUNT_LIST_KEY: B256 = hash_string("SUBACCOUNT_LIST");
pub const MAX_ALLOWED_SUBACCOUNT_ACTION_COUNT: B256 = hash_string("MAX_ALLOWED_SUBACCOUNT_ACTION_COUNT");
pub const SUBACCOUNT_ACTION_COUNT: B256 = hash_string("SUBACCOUNT_ACTION_COUNT");
pub const SUBACCOUNT_ORDER_ACTION: B256 = hash_string("SUBACCOUNT_ORDER_ACTION");
pub const SUBACCOUNT_AUTO_TOP_UP_AMOUNT: B256 = hash_string("SUBACCOUNT_AUTO_TOP_UP_AMOUNT");
pub const INDEX_TOKENS_LIST_KEY: B256 = hash_string("INDEX_TOKENS_LIST");

// Key generation functions
pub fn position_impact_factor_key(market: &str, is_positive: bool) -> B256 {
    hash_data(&["bytes32", "address", "bool"], &[POSITION_IMPACT_FACTOR_KEY, market, is_positive])
}

pub fn position_impact_factor_key_for_dynamic_market(market: &str, index_token: &str, is_positive: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[POSITION_IMPACT_FACTOR_KEY, market, index_token, is_positive]
    )
}

pub fn position_impact_exponent_factor_key(market: &str) -> B256 {
    hash_data(&["bytes32", "address"], &[POSITION_IMPACT_EXPONENT_FACTOR_KEY, market])
}

pub fn position_impact_exponent_factor_key_for_dynamic_market(market: &str, index_token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[POSITION_IMPACT_EXPONENT_FACTOR_KEY, market, index_token]
    )
}

pub fn max_position_impact_factor_key(market: &str, is_positive: bool) -> B256 {
    hash_data(&["bytes32", "address", "bool"], &[MAX_POSITION_IMPACT_FACTOR_KEY, market, is_positive])
}

pub fn max_position_impact_factor_key_for_dynamic_market(market: &str, index_token: &str, is_positive: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[MAX_POSITION_IMPACT_FACTOR_KEY, market, index_token, is_positive]
    )
}

pub fn position_fee_factor_key(market: &str, for_positive_impact: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[POSITION_FEE_FACTOR_KEY, market, for_positive_impact]
    )
}

pub fn position_fee_factor_key_for_dynamic_market(market: &str, index_token: &str, for_positive_impact: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[POSITION_FEE_FACTOR_KEY, market, index_token, for_positive_impact]
    )
}

pub fn swap_impact_factor_key(market: &str, is_positive: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[SWAP_IMPACT_FACTOR_KEY, market, is_positive]
    )
}

pub fn swap_impact_exponent_factor_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[SWAP_IMPACT_EXPONENT_FACTOR_KEY, market]
    )
}

pub fn swap_fee_factor_key(market: &str, for_positive_impact: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[SWAP_FEE_FACTOR_KEY, market, for_positive_impact]
    )
}

pub fn open_interest_in_tokens_key(market: &str, collateral_token: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[OPEN_INTEREST_IN_TOKENS_KEY, market, collateral_token, is_long]
    )
}

pub fn open_interest_in_tokens_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    collateral_token: &str, 
    is_long: bool
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "address", "bool"],
        &[OPEN_INTEREST_IN_TOKENS_KEY, market, index_token, collateral_token, is_long]
    )
}

pub fn pool_amount_key(market: &str, token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[POOL_AMOUNT_KEY, market, token]
    )
}

pub fn pool_amount_key_for_dynamic_market(market: &str, index: &str, token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "address"],
        &[POOL_AMOUNT_KEY, market, index, token]
    )
}

pub fn reserve_factor_key(market: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[RESERVE_FACTOR_KEY, market, is_long]
    )
}

pub fn reserve_factor_key_for_dynamic_market(market: &str, index_token: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[RESERVE_FACTOR_KEY, market, index_token, is_long]
    )
}

pub fn open_interest_reserve_factor_key(market: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[OPEN_INTEREST_RESERVE_FACTOR_KEY, market, is_long]
    )
}

pub fn open_interest_reserve_factor_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    is_long: bool
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[OPEN_INTEREST_RESERVE_FACTOR_KEY, market, index_token, is_long]
    )
}

pub fn max_open_interest_key(market: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[MAX_OPEN_INTEREST_KEY, market, is_long]
    )
}

pub fn max_open_interest_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    long_token_address: &str, 
    is_long: bool
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "address", "bool"],
        &[MAX_OPEN_INTEREST_KEY, market, index_token, long_token_address, is_long]
    )
}

pub fn borrowing_factor_key(market: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[BORROWING_FACTOR_KEY, market, is_long]
    )
}

pub fn borrowing_factor_key_for_dynamic_market(market: &str, index_token: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[BORROWING_FACTOR_KEY, market, index_token, is_long]
    )
}

pub fn borrowing_exponent_factor_key(market: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[BORROWING_EXPONENT_FACTOR_KEY, market, is_long]
    )
}

pub fn borrowing_exponent_factor_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    is_long: bool
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[BORROWING_EXPONENT_FACTOR_KEY, market, index_token, is_long]
    )
}

pub fn cumulative_borrowing_factor_key(market: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[CUMULATIVE_BORROWING_FACTOR_KEY, market, is_long]
    )
}

pub fn cumulative_borrowing_factor_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    is_long: bool
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[CUMULATIVE_BORROWING_FACTOR_KEY, market, index_token, is_long]
    )
}

pub fn total_borrowing_key(market: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[TOTAL_BORROWING_KEY, market, is_long]
    )
}

pub fn total_borrowing_key_for_dynamic_market(market: &str, index_token: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[TOTAL_BORROWING_KEY, market, index_token, is_long]
    )
}

pub fn funding_factor_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[FUNDING_FACTOR_KEY, market]
    )
}

pub fn funding_factor_key_for_dynamic_market(market: &str, index_token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[FUNDING_FACTOR_KEY, market, index_token]
    )
}

pub fn funding_exponent_factor_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[FUNDING_EXPONENT_FACTOR_KEY, market]
    )
}

pub fn funding_exponent_factor_key_for_dynamic_market(market: &str, index_token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[FUNDING_EXPONENT_FACTOR_KEY, market, index_token]
    )
}

pub fn funding_increase_factor_per_second_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[FUNDING_INCREASE_FACTOR_PER_SECOND, market]
    )
}

pub fn funding_increase_factor_per_second_for_dynamic_market_key(
    market: &str, 
    index_token: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[FUNDING_INCREASE_FACTOR_PER_SECOND, market, index_token]
    )
}

pub fn funding_decrease_factor_per_second_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[FUNDING_DECREASE_FACTOR_PER_SECOND, market]
    )
}

pub fn funding_decrease_factor_per_second_for_dynamic_market_key(
    market: &str, 
    index_token: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[FUNDING_DECREASE_FACTOR_PER_SECOND, market, index_token]
    )
}

pub fn min_funding_factor_per_second_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[MIN_FUNDING_FACTOR_PER_SECOND, market]
    )
}

pub fn min_funding_factor_per_second_for_dynamic_market_key(
    market: &str, 
    index_token: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[MIN_FUNDING_FACTOR_PER_SECOND, market, index_token]
    )
}

pub fn max_funding_factor_per_second_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[MAX_FUNDING_FACTOR_PER_SECOND, market]
    )
}

pub fn max_funding_factor_per_second_for_dynamic_market_key(
    market: &str, 
    index_token: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[MAX_FUNDING_FACTOR_PER_SECOND, market, index_token]
    )
}

pub fn threshold_for_stable_funding_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[THRESHOLD_FOR_STABLE_FUNDING, market]
    )
}

pub fn threshold_for_stable_funding_key_for_dynamic_market(
    market: &str, 
    index_token: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[THRESHOLD_FOR_STABLE_FUNDING, market, index_token]
    )
}

pub fn threshold_for_decrease_funding_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[THRESHOLD_FOR_DECREASE_FUNDING, market]
    )
}

pub fn threshold_for_decrease_funding_key_dynamic_market(
    market: &str, 
    index_token: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[THRESHOLD_FOR_DECREASE_FUNDING, market, index_token]
    )
}

pub fn max_pnl_factor_key(pnl_factor_type: &str, market: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "bytes32", "address", "bool"],
        &[MAX_PNL_FACTOR_KEY, pnl_factor_type, market, is_long]
    )
}

pub fn max_pnl_factor_key_for_dynamic_market(
    pnl_factor_type: &str, 
    market: &str, 
    index_token: &str, 
    is_long: bool
) -> B256 {
    hash_data(
        &["bytes32", "bytes32", "address", "address", "bool"],
        &[MAX_PNL_FACTOR_KEY, pnl_factor_type, market, index_token, is_long]
    )
}

pub fn position_impact_pool_amount_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[POSITION_IMPACT_POOL_AMOUNT_KEY, market]
    )
}

pub fn position_impact_pool_amount_key_for_dynamic_market(market: &str, index_token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[POSITION_IMPACT_POOL_AMOUNT_KEY, market, index_token]
    )
}

pub fn min_position_impact_pool_amount_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[MIN_POSITION_IMPACT_POOL_AMOUNT_KEY, market]
    )
}

pub fn min_position_impact_pool_amount_key_for_dynamic_market(market: &str, index_token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[MIN_POSITION_IMPACT_POOL_AMOUNT_KEY, market, index_token]
    )
}

pub fn position_impact_pool_distribution_rate_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[POSITION_IMPACT_POOL_DISTRIBUTION_RATE_KEY, market]
    )
}

pub fn max_position_impact_factor_for_liquidations_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[MAX_POSITION_IMPACT_FACTOR_FOR_LIQUIDATIONS_KEY, market]
    )
}

pub fn max_position_impact_factor_key_for_liquidations_for_dynamic_market(
    market: &str, 
    index_token: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[MAX_POSITION_IMPACT_FACTOR_FOR_LIQUIDATIONS_KEY, market, index_token]
    )
}

pub fn swap_impact_pool_amount_key(market: &str, token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[SWAP_IMPACT_POOL_AMOUNT_KEY, market, token]
    )
}

pub fn min_collateral_factor_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[MIN_COLLATERAL_FACTOR_KEY, market]
    )
}

pub fn min_collateral_factor_key_for_dynamic_market(market: &str, index_token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[MIN_COLLATERAL_FACTOR_KEY, market, index_token]
    )
}

pub fn min_collateral_factor_for_open_interest(market: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "bool"],
        &[MIN_COLLATERAL_FACTOR_FOR_OPEN_INTEREST_MULTIPLIER_KEY, market, is_long]
    )
}

pub fn min_collateral_factor_for_open_interest_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    is_long: bool
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[MIN_COLLATERAL_FACTOR_FOR_OPEN_INTEREST_MULTIPLIER_KEY, market, index_token, is_long]
    )
}

pub fn claimable_funding_amount_key(market: &str, token: &str, account: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "address"],
        &[CLAIMABLE_FUNDING_AMOUNT, market, token, account]
    )
}

pub fn claimable_funding_amount_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    token: &str, 
    account: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "address", "address"],
        &[CLAIMABLE_FUNDING_AMOUNT, market, index_token, token, account]
    )
}

pub fn virtual_token_id_key(token: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[VIRTUAL_TOKEN_ID_KEY, token]
    )
}

pub fn virtual_token_id_key_for_dynamic_market(
    token: &str, 
    index_token: &str, 
    collateral_token: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "address"],
        &[VIRTUAL_TOKEN_ID_KEY, token, index_token, collateral_token]
    )
}

pub fn virtual_market_id_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[VIRTUAL_MARKET_ID_KEY, market]
    )
}

pub fn virtual_inventory_for_swaps_key(virtual_market_id: &str, token: &str) -> B256 {
    hash_data(
        &["bytes32", "bytes32", "address"],
        &[VIRTUAL_INVENTORY_FOR_SWAPS_KEY, virtual_market_id, token]
    )
}

pub fn virtual_inventory_for_positions_key(virtual_token_id: &str) -> B256 {
    hash_data(
        &["bytes32", "bytes32"],
        &[VIRTUAL_INVENTORY_FOR_POSITIONS_KEY, virtual_token_id]
    )
}

pub fn pool_amount_adjustment_key(market: &str, token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[POOL_AMOUNT_ADJUSTMENT_KEY, market, token]
    )
}

pub fn affiliate_reward_key(market: &str, token: &str, account: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "address"],
        &[AFFILIATE_REWARD_KEY, market, token, account]
    )
}

pub fn is_market_disabled_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[IS_MARKET_DISABLED_KEY, market]
    )
}

pub fn max_pool_amount_for_deposit_key(market: &str, token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[MAX_POOL_AMOUNT_FOR_DEPOSIT_KEY, market, token]
    )
}

pub fn max_pool_amount_for_deposit_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    token: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "address"],
        &[MAX_POOL_AMOUNT_FOR_DEPOSIT_KEY, market, index_token, token]
    )
}

pub fn max_pool_amount_key(market: &str, token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[MAX_POOL_AMOUNT_KEY, market, token]
    )
}

pub fn max_pool_amount_key_for_dynamic_market(market: &str, index_token: &str, token: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "address"],
        &[MAX_POOL_AMOUNT_KEY, market, index_token, token]
    )
}

pub fn ui_fee_factor_key(address: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[UI_FEE_FACTOR, address]
    )
}

pub fn subaccount_list_key(account: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[SUBACCOUNT_LIST_KEY, account]
    )
}

pub fn max_allowed_subaccount_action_count_key(
    account: &str, 
    subaccount: &str, 
    action_type: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bytes32"],
        &[MAX_ALLOWED_SUBACCOUNT_ACTION_COUNT, account, subaccount, action_type]
    )
}

pub fn subaccount_action_count_key(
    account: &str, 
    subaccount: &str, 
    action_type: &str
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bytes32"],
        &[SUBACCOUNT_ACTION_COUNT, account, subaccount, action_type]
    )
}

pub fn subaccount_auto_top_up_amount_key(account: &str, subaccount: &str) -> B256 {
    hash_data(
        &["bytes32", "address", "address"],
        &[SUBACCOUNT_AUTO_TOP_UP_AMOUNT, account, subaccount]
    )
}

pub fn index_tokens_list_key(market: &str) -> B256 {
    hash_data(
        &["bytes32", "address"],
        &[INDEX_TOKENS_LIST_KEY, market]
    )
}

pub fn open_interest_key(market: &str, collateral_token: &str, is_long: bool) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "bool"],
        &[OPEN_INTEREST_KEY, market, collateral_token, is_long]
    )
}

pub fn open_interest_key_for_dynamic_market(
    market: &str, 
    index_token: &str, 
    collateral_token: &str, 
    is_long: bool
) -> B256 {
    hash_data(
        &["bytes32", "address", "address", "address", "bool"],
        &[OPEN_INTEREST_KEY, market, index_token, collateral_token, is_long]
    )
}

pub fn hashed_position_key(account: &str, market: &str, collateral_token: &str, is_long: bool) -> B256 {
    hash_data(
        &["address", "address", "address", "bool"],
        &[account, market, collateral_token, is_long]
    )
}

pub fn hashed_position_key_for_dynamic_market(
    account: &str, 
    market: &str, 
    index_token: &str, 
    collateral_token: &str, 
    is_long: bool
) -> B256 {
    hash_data(
        &["address", "address", "address", "address", "bool"],
        &[account, market, index_token, collateral_token, is_long]
    )
}

pub fn order_key(data_store_address: &str, nonce: u64) -> B256 {
    hash_data(&["address", "uint256"], &[data_store_address, nonce])
}

pub fn deposit_gas_limit_key(single_token: bool) -> B256 {
    hash_data(&["bytes32", "bool"], &[DEPOSIT_GAS_LIMIT_KEY, single_token])
}

pub fn withdrawal_gas_limit_key() -> B256 {
    hash_data(&["bytes32"], &[WITHDRAWAL_GAS_LIMIT_KEY])
}

pub fn single_swap_gas_limit_key() -> B256 {
    SINGLE_SWAP_GAS_LIMIT_KEY
}

pub fn increase_order_gas_limit_key() -> B256 {
    INCREASE_ORDER_GAS_LIMIT_KEY
}

pub fn decrease_order_gas_limit_key() -> B256 {
    DECREASE_ORDER_GAS_LIMIT_KEY
}

pub fn swap_order_gas_limit_key() -> B256 {
    SWAP_ORDER_GAS_LIMIT_KEY
}

pub fn account_order_list_key(account: &str) -> B256 {
    hash_data(&["bytes32", "address"], &[ACCOUNT_ORDER_LIST_KEY, account])
}

pub fn account_position_list_key(account: &str) -> B256 {
    hash_data(&["bytes32", "address"], &[ACCOUNT_POSITION_LIST_KEY, account])
}