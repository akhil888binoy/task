use alloy::primitives::{keccak256, B256, U256};
use alloy::sol_types::SolValue;

/// Hashes encoded Solidity data
pub fn hash_data<T: SolValue>(data: &T) -> B256 {
    let encoded = data.abi_encode(); // Encode using Alloy's SolValue trait
    keccak256(encoded)
}

pub  fn hash_string(s: &str) -> B256 {
    let encoded = SolValue::abi_encode(s);
    keccak256(encoded)
}

/// Expands a number with `decimals` precision
pub fn expand_decimals(n: U256, decimals: u32) -> U256 {
    n.checked_mul(U256::from(10).pow(U256::from(decimals)))
        .expect("Overflow in expand_decimals")
}

/// Parses a contract price based on token decimals
pub fn parse_contract_price(price: U256, token_decimals: u32) -> U256 {
    price.checked_mul(expand_decimals(U256::from(1), token_decimals))
        .expect("Overflow in parse_contract_price")
}

