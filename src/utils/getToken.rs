

use crate::config::chainConfig::BITLAYER;
use alloy::primitives::{Address, U256};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::model::token::Token;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Token {
//     pub name: String,
//     pub market_type: Option<String>,
//     pub symbol: String,
//     pub asset_symbol: Option<String>,
//     pub base_symbol: Option<String>,
//     pub decimals: u8,
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
    
//     pub is_usdg: bool,
//     pub is_native: bool,
//     pub is_wrapped: bool,
//     pub is_shortable: bool,
//     pub is_stable: bool,
//     pub is_synthetic: bool,
//     pub is_temp_hidden: bool,
//     pub is_chart_disabled: bool,
//     pub is_v1_available: bool,
//     pub is_platform_token: bool,
//     pub is_platform_trading_token: bool,
// }

// impl Default for Token {
//     fn default() -> Self {
//         Self {
//             name: String::new(),
//             market_type: None,
//             symbol: String::new(),
//             asset_symbol: None,
//             base_symbol: None,
//             decimals: 18,
//             address: Address::ZERO,
//             price_decimals: None,
//             wrapped_address: None,
//             coingecko_url: None,
//             coingecko_symbol: None,
//             metamask_symbol: None,
//             explorer_symbol: None,
//             explorer_url: None,
//             reserves_url: None,
//             image_url: None,
//             is_usdg: false,
//             is_native: false,
//             is_wrapped: false,
//             is_shortable: false,
//             is_stable: false,
//             is_synthetic: false,
//             is_temp_hidden: false,
//             is_chart_disabled: false,
//             is_v1_available: false,
//             is_platform_token: false,
//             is_platform_trading_token: false,
//         }
//     }
// }

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
            address: "0x852519aCe4F8ec20Da1B940680765ab13ac69eC0".parse().unwrap(),
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
            address: "0x85a41e742868f3Ba0930405100DCA46FB9CAeE89".parse().unwrap(),
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
            address: "0x501b9Af9d9C41c374Acc419FbeE07f284a2c351B".parse().unwrap(),
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
            address: "0x13AE4c71342Be5483bF5ab3bA29714cAA11E4ad9".parse().unwrap(),
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
            address: "0x46924277A232b34F6CC0bF3271C8d49Cb2a93FAF".parse().unwrap(),
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
            address: "0xf74955AaFe0d6EBd096a05686E7AE170CE0bb291".parse().unwrap(),
            image_url: Some("https://assets.coingecko.com/coins/images/7598/thumb/wrapped_bitcoin_wbtc.png?1548822744".to_string()),
            coingecko_url: Some("https://www.coingecko.com/en/coins/wrapped-bitcoin".to_string()),
            explorer_url: Some("https://testnet.snowtrace.io/address/0x3Bd8e00c25B12E6E60fc8B6f1E1E2236102073Ca".to_string()),
            ..Default::default()
        },
    ]);
    
    map
});

pub static V2_TOKENS: Lazy<HashMap<u32, Vec<Token>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    for (chain_id, tokens) in TOKENS.iter() {
        let v2_tokens: Vec<Token> = tokens.iter()
            .filter(|token| token.is_platform_token || token.is_v1_available)
            .cloned()
            .collect();
            
        map.insert(*chain_id, if v2_tokens.is_empty() {
            tokens.clone()
        } else {
            v2_tokens
        });
    }
    
    map
});

pub static WRAPPED_TOKENS_MAP: Lazy<HashMap<u32, Token>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    for (chain_id, tokens) in TOKENS.iter() {
        if let Some(wrapped_token) = tokens.iter().find(|token| token.is_wrapped) {
            map.insert(*chain_id, wrapped_token.clone());
        } else {
            log::warn!("No wrapped token found for chainId {}", chain_id);
        }
    }
    
    map
});

pub static TOKENS_MAP: Lazy<HashMap<u32, HashMap<String, Token>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    for (chain_id, tokens) in TOKENS.iter() {
        let tokens_map: HashMap<String, Token> = tokens.iter()
            .map(|token| (token.address.to_string().to_lowercase(), token.clone()))
            .collect();
            
        map.insert(*chain_id, tokens_map);
    }
    
    map
});

pub fn get_tokens_map(chain_id: u32) -> Option<&'static HashMap<String, Token>> {
    TOKENS_MAP.get(&chain_id)
}

pub fn get_v2_tokens(chain_id: u32) -> Option<&'static Vec<Token>> {
    V2_TOKENS.get(&chain_id)
}

pub fn get_token(chain_id: u32, address: &str) -> Result<&'static Token, String> {
    let address_lower = address.to_lowercase();
    TOKENS_MAP.get(&chain_id)
        .and_then(|tokens| tokens.get(&address_lower))
        .ok_or_else(|| format!("Token not found for address {} on chain {}", address, chain_id))
}

pub fn get_wrapped_token(chain_id: u32) -> Option<&'static Token> {
    WRAPPED_TOKENS_MAP.get(&chain_id)
}

#[derive(Debug)]
pub struct TokenBalancesData {
    pub balances: HashMap<Address, U256>,
}
