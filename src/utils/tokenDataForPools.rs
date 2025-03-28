
use alloy::primitives::Address;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::config::chainConfig::BITLAYER;
use crate::utils::helpers::NATIVE_TOKEN_ADDRESS;
use crate::types::types::{TokensData, TokenData};
use crate::model::token::Token;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenVersion {
    pub is_synthetic: Option<bool>,
    pub version: Option<String>, 
}

pub static V1_TOKENS: Lazy<HashMap<u32, Vec<Token>>> = Lazy::new(|| HashMap::new());
pub static V2_TOKENS: Lazy<HashMap<u32, Vec<Token>>> = Lazy::new(|| HashMap::new());

pub static TOKENS: Lazy<HashMap<u32, Vec<Token>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    map.insert(BITLAYER, vec![
        Token {
            name: "BTC".to_string(),
            symbol: "BTC".to_string(),
            decimals: 18,
            address: Address::ZERO,
            is_native: true,
            is_shortable: true,
            image_url: Some("https://assets.coingecko.com/coins/images/12559/small/coin-round-red.png?1604021818".to_string()),
            ..Default::default()
        },
        Token {
            name: "Wrapped BTC".to_string(),
            symbol: "WBTC".to_string(),
            decimals: 18,
            address: "0x313ea66A1f508B5F2825A626F7a09afeaBE594E1".parse().unwrap(),
            is_wrapped: true,
            base_symbol: Some("BTC".to_string()),
            image_url: Some("https://assets.coingecko.com/coins/images/12559/small/coin-round-red.png?1604021818".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/avalanche".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x1D308089a2D1Ced3f1Ce36B1FcaF815b07217be3".to_string()),
            ..Default::default()
        },
        Token {
            name: "USD Coin".to_string(),
            symbol: "USDC".to_string(),
            address: "0xdaA0775cbC0b25Bfae94B5F1dB5956d03761EB3c".parse().unwrap(),
            decimals: 6,
            is_stable: true,
            image_url: Some("https://assets.coingecko.com/coins/images/6319/thumb/USD_Coin_icon.png?1547042389".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/usd-coin".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x3eBDeaA0DB3FfDe96E7a0DBBAFEC961FC50F725F".to_string()),
            ..Default::default()
        },
        Token {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            address: "0x82aAd3bDab1c2F2FC12EdFd8F573639bF91bdd55".parse().unwrap(),
            is_shortable: true,
            image_url: Some("https://assets.coingecko.com/coins/images/325/small/Tether-logo.png".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/dai".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x50df4892Bd13f01E4e1Cd077ff394A8fa1A3fD7c".to_string()),
            ..Default::default()
        },
        Token {
            name: "Solana".to_string(),
            symbol: "SOL".to_string(),
            decimals: 18,
            address: "0xB7a11320D5B046a0dF3c4e79eba18Cf8015b98a4".parse().unwrap(),
            image_url: Some("https://assets.coingecko.com/coins/images/7598/thumb/wrapped_bitcoin_wbtc.png?1548822744".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/wrapped-bitcoin".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x3Bd8e00c25B12E6E60fc8B6f1E1E2236102073Ca".to_string()),
            ..Default::default()
        },
        Token {
            name: "Hyperliquid".to_string(),
            symbol: "HYPE".to_string(),
            decimals: 18,
            is_shortable: true,
            address: "0xf81432D3031B237F7fe320896eC9BB4505D95b31".parse().unwrap(),
            image_url: Some("https://assets.coingecko.com/coins/images/7598/thumb/wrapped_bitcoin_wbtc.png?1548822744".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/wrapped-bitcoin".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x3Bd8e00c25B12E6E60fc8B6f1E1E2236102073Ca".to_string()),
            ..Default::default()
        },
        Token {
            name: "Pendle".to_string(),
            symbol: "PENDLE".to_string(),
            decimals: 18,
            is_shortable: true,
            address: "0x50493DDF2188caa0A4194cdeF1E9144c8fFf2F45".parse().unwrap(),
            image_url: Some("https://assets.coingecko.com/coins/images/7598/thumb/wrapped_bitcoin_wbtc.png?1548822744".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/wrapped-bitcoin".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x3Bd8e00c25B12E6E60fc8B6f1E1E2236102073Ca".to_string()),
            ..Default::default()
        },
    ]);
    
    map
});

pub static TOKENS_MAP: Lazy<HashMap<u32, HashMap<Address, Token>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    for (&chain_id, tokens) in TOKENS.iter() {
        let mut chain_map = HashMap::new();
        for token in tokens {
            chain_map.insert(token.address, token.clone());
        }
        map.insert(chain_id, chain_map);
    }
    
    map
});

pub static TOKENS_BY_SYMBOL_MAP: Lazy<HashMap<u32, HashMap<String, Token>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    for (&chain_id, tokens) in TOKENS.iter() {
        let mut symbol_map = HashMap::new();
        for token in tokens {
            symbol_map.insert(token.symbol.to_lowercase(), token.clone());
        }
        map.insert(chain_id, symbol_map);
    }
    
    map
});

pub fn get_v1_tokens(chain_id: u32) -> Option<&'static Vec<Token>> {
    V1_TOKENS.get(&chain_id)
}

pub fn get_v2_tokens(chain_id: u32) -> Vec<Token> {
    V2_TOKENS.get(&chain_id)
        .map(|tokens| tokens.clone())
        .unwrap_or_else(|| TOKENS.get(&chain_id).cloned().unwrap_or_default())
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Token with symbol {0} not found for chainId {1}")]
    TokenNotFound(String, u32),
}
pub fn get_token_by_symbol(
    chain_id: u32,
    symbol: &str,
    options: Option<TokenVersion>,
) -> Result<&'static Token, TokenError> {
    // First check TOKENS_BY_SYMBOL_MAP which contains static references
    if let Some(token) = TOKENS_BY_SYMBOL_MAP
        .get(&chain_id)
        .and_then(|m| m.get(&symbol.to_lowercase()))
    {
        return Ok(token);
    }

    let is_synthetic = options.as_ref().and_then(|o| o.is_synthetic).unwrap_or(false);

    // Handle version-specific tokens
    match options.as_ref().and_then(|o| o.version.as_deref()) {
        Some("v1") => {
            if let Some(tokens) = get_v1_tokens(chain_id) {
                if let Some(token) = find_token_in_list(tokens, symbol, is_synthetic) {
                    return Ok(token);
                }
            }
        }
        Some("v2") => {
            // For v2, we need to use the static TOKENS map if V2_TOKENS is empty
            let tokens = if let Some(v2_tokens) = V2_TOKENS.get(&chain_id) {
                v2_tokens
            } else {
                match TOKENS.get(&chain_id) {
                    Some(t) => t,
                    None => return Err(TokenError::TokenNotFound(symbol.to_string(), chain_id)),
                }
            };
            
            if let Some(token) = find_token_in_list(tokens, symbol, is_synthetic) {
                return Ok(token);
            }
        }
        _ => {
            // Default case - use TOKENS
            if let Some(tokens) = TOKENS.get(&chain_id) {
                if let Some(token) = find_token_in_list(tokens, symbol, is_synthetic) {
                    return Ok(token);
                }
            }
        }
    }

    Err(TokenError::TokenNotFound(symbol.to_string(), chain_id))
}

// Helper function to find token in a list
fn find_token_in_list<'a>(
    tokens: &'a [Token],
    symbol: &str,
    is_synthetic: bool,
) -> Option<&'a Token> {
    if is_synthetic {
        tokens.iter()
            .find(|t| t.symbol.eq_ignore_ascii_case(symbol) && t.is_synthetic)
    } else {
        tokens.iter()
            .find(|t| t.symbol.eq_ignore_ascii_case(symbol))
    }
}
pub fn get_token_data(
    tokens_data: &TokensData,
    address: Option<Address>,
    convert_to: Option<&str>,
) -> Option<TokenData> {
    let address = address?;
    let token = tokens_data.get(&address)?.clone();  

    match convert_to {
        Some("wrapped") if token.token.is_native => {
            token.token.wrapped_address
                .and_then(|addr| tokens_data.get(&addr).cloned())
        }
        Some("native") if token.token.is_wrapped => {
            tokens_data.get(&NATIVE_TOKEN_ADDRESS).cloned()
        }
        _ => Some(token),
    }
}