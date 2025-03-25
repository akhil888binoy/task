#[path = "../config/chainConfig.rs"]
mod chain_config;
#[path = "./get_token.rs"]
mod get_token;
#[path = "./helpers.rs"]
mod helpers;
#[path = "../types.rs"]
mod types;

use alloy::primitives::Address;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use chain_config::MOVEMENT_DEVNET_CHAIN_ID;
use helpers::NATIVE_TOKEN_ADDRESS;
use types::TokensData;
use get_token::Token;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenVersion {
    pub is_synthetic: Option<bool>,
    pub version: Option<String>, 
}

pub static V1_TOKENS: Lazy<HashMap<u32, Vec<Token>>> = Lazy::new(|| HashMap::new());
pub static V2_TOKENS: Lazy<HashMap<u32, Vec<Token>>> = Lazy::new(|| HashMap::new());

pub static TOKENS: Lazy<HashMap<u32, Vec<Token>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    map.insert(MOVEMENT_DEVNET_CHAIN_ID, vec![
        Token {
            name: "Move".to_string(),
            symbol: "MOVE".to_string(),
            decimals: 18,
            address: Address::ZERO,
            is_native: true,
            is_shortable: true,
            image_url: Some("https://assets.coingecko.com/coins/images/12559/small/coin-round-red.png?1604021818".to_string()),
            ..Default::default()
        },
        Token {
            name: "Wrapped Move".to_string(),
            symbol: "WMOVE".to_string(),
            decimals: 18,
            address: "0x47759683E8789130571673C2968C8F4b9f22c251".parse().unwrap(),
            is_wrapped: true,
            base_symbol: Some("MOVE".to_string()),
            image_url: Some("https://assets.coingecko.com/coins/images/12559/small/coin-round-red.png?1604021818".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/avalanche".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x1D308089a2D1Ced3f1Ce36B1FcaF815b07217be3".to_string()),
            ..Default::default()
        },
        Token {
            name: "Wrapped Bitcoin".to_string(),
            symbol: "WBTC".to_string(),
            address: "0xEb3c2e768c17E0c2AFF98bdF0024D38A18b0B62E".parse().unwrap(),
            decimals: 8,
            is_shortable: true,
            image_url: Some("https://assets.coingecko.com/coins/images/279/small/ethereum.png?1595348880".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/weth".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x82F0b3695Ed2324e55bbD9A9554cB4192EC3a514".to_string()),
            ..Default::default()
        },
        Token {
            name: "USD Coin".to_string(),
            symbol: "USDC".to_string(),
            address: "0x38604D543659121faa8F68A91A5b633C7BFE9761".parse().unwrap(),
            decimals: 6,
            is_stable: true,
            image_url: Some("https://assets.coingecko.com/coins/images/6319/thumb/USD_Coin_icon.png?1547042389".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/usd-coin".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x3eBDeaA0DB3FfDe96E7a0DBBAFEC961FC50F725F".to_string()),
            ..Default::default()
        },
        Token {
            name: "Wrapped Ether".to_string(),
            symbol: "WETH".to_string(),
            decimals: 18,
            address: "0xd778B815E6AE26f547042bbbe4Bf8b1B0c746A22".parse().unwrap(),
            is_shortable: true,
            image_url: Some("https://assets.coingecko.com/coins/images/325/small/Tether-logo.png".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/dai".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x50df4892Bd13f01E4e1Cd077ff394A8fa1A3fD7c".to_string()),
            ..Default::default()
        },
        Token {
            name: "Wrapped Staked Ether".to_string(),
            symbol: "WSTETH".to_string(),
            decimals: 18,
            address: "0xeAC3d56DCB15a3Bc174aB292B7023e9Fc9F7aDf0".parse().unwrap(),
            image_url: Some("https://assets.coingecko.com/coins/images/7598/thumb/wrapped_bitcoin_wbtc.png?1548822744".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/wrapped-bitcoin".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x3Bd8e00c25B12E6E60fc8B6f1E1E2236102073Ca".to_string()),
            ..Default::default()
        },
        Token {
            name: "Milkway Staked TIA".to_string(),
            symbol: "MILKTIA".to_string(),
            decimals: 18,
            address: "0x2A197C29f3E144387EB5877CFe0e63032FD1a0DA".parse().unwrap(),
            image_url: Some("https://assets.coingecko.com/coins/images/7598/thumb/wrapped_bitcoin_wbtc.png?1548822744".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/wrapped-bitcoin".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x3Bd8e00c25B12E6E60fc8B6f1E1E2236102073Ca".to_string()),
            ..Default::default()
        },
        Token {
            name: "Staked Move".to_string(),
            symbol: "STMOVE".to_string(),
            decimals: 18,
            is_shortable: true,
            address: "0x1AD94D0a799664D459cB467655eC0EA4cc8Ad478".parse().unwrap(),
            image_url: Some("https://assets.coingecko.com/coins/images/7598/thumb/wrapped_bitcoin_wbtc.png?1548822744".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/wrapped-bitcoin".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x3Bd8e00c25B12E6E60fc8B6f1E1E2236102073Ca".to_string()),
            ..Default::default()
        },
        Token {
            name: "GoGoPool AVAX".to_string(),
            symbol: "GGAVAX".to_string(),
            decimals: 18,
            is_shortable: true,
            address: "0xb9aDf17948481eb380D37E9594fD4382372DBcd0".parse().unwrap(),
            image_url: Some("https://assets.coingecko.com/coins/images/7598/thumb/wrapped_bitcoin_wbtc.png?1548822744".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/wrapped-bitcoin".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x3Bd8e00c25B12E6E60fc8B6f1E1E2236102073Ca".to_string()),
            ..Default::default()
        },
        Token {
            name: "Optimus Market tokens".to_string(),
            symbol: "OM".to_string(),
            address: "<market-token-address>".parse().unwrap(),
            decimals: 18,
            image_url: Some("0xabin".to_string()),
            is_platform_token: true,
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
    let tokens = match options.as_ref().and_then(|o| o.version.as_ref()) {
        Some(v) if v == "v1" => get_v1_tokens(chain_id),
        Some(v) if v == "v2" => Some(&get_v2_tokens(chain_id)),
        _ => TOKENS.get(&chain_id),
    };

    let is_synthetic = options.map(|o| o.is_synthetic.unwrap_or(false)).unwrap_or(false);

    if let Some(tokens) = tokens {
        if is_synthetic {
            if let Some(token) = tokens.iter().find(|t| {
                t.symbol.eq_ignore_ascii_case(symbol) && t.is_synthetic.unwrap_or(false)
            }) {
                return Ok(token);
            }
        }

        if let Some(token) = tokens.iter().find(|t| t.symbol.eq_ignore_ascii_case(symbol)) {
            return Ok(token);
        }
    }

    if let Some(token) = TOKENS_BY_SYMBOL_MAP
        .get(&chain_id)
        .and_then(|m| m.get(&symbol.to_lowercase()))
    {
        return Ok(token);
    }

    Err(TokenError::TokenNotFound(symbol.to_string(), chain_id))
}

pub fn get_token_data(
    tokens_data: &TokensData,
    address: Option<Address>,
    convert_to: Option<&str>,
) -> Option<&Token> {
    let address = address?;
    let token = tokens_data.get(&address)?;

    match convert_to {
        Some("wrapped") if token.is_native => token.wrapped_address.and_then(|addr| tokens_data.get(&addr)),
        Some("native") if token.is_wrapped => tokens_data.get(&NATIVE_TOKEN_ADDRESS),
        _ => Some(token),
    }
}