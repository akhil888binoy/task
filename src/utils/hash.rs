use alloy::primitives::{keccak256, B256, U256};
use alloy::sol_types::{SolValue, SolType};
use alloy::sol;

// Equivalent to hashData function
pub fn hash_data(data_types: &[&str], data_values: &[&dyn SolValue]) -> B256 {
    // Note: In practice you'd want to properly map types to SolTypes
    let encoded = alloy::SolCall::abi_encode_params(data_types, data_values);
    keccak256(encoded)
}

// Equivalent to hashString function
pub fn hash_string(s: &str) -> B256 {
    // Using alloy's built-in string encoding
    let encoded = s.encode_sol();
    keccak256(encoded)
}

// Equivalent to expandDecimals function
pub fn expand_decimals(n: U256, decimals: u32) -> U256 {
    n.checked_mul(U256::from(10).pow(U256::from(decimals)))
        .expect("Overflow in expand_decimals")
}

// Equivalent to parseContractPrice function
pub fn parse_contract_price(price: U256, token_decimals: u32) -> U256 {
    price.checked_mul(expand_decimals(U256::from(1), token_decimals))
        .expect("Overflow in parse_contract_price")
}

// Helper trait for SolValue objects
pub trait SolValueHelpers {
    fn encode_sol(&self) -> Vec<u8>;
}

impl SolValueHelpers for str {
    fn encode_sol(&self) -> Vec<u8> {
        alloy::sol_types::SolString::encode_sol(self)
    }
}

// // Example usage
// fn main() {
//     // Hash data example
//     let hash = hash_data(&["uint256", "address"], &[&U256::from(42), &Address::ZERO]);
//     println!("Hashed data: {:?}", hash);

//     // Hash string example
//     let string_hash = hash_string("hello world");
//     println!("String hash: {:?}", string_hash);

//     // Expand decimals example
//     let expanded = expand_decimals(U256::from(5), 18);
//     println!("5 with 18 decimals: {}", expanded);

//     // Parse contract price example
//     let price = parse_contract_price(U256::from(100), 6);
//     println!("Price with decimals: {}", price);
// }