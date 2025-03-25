#[path = "../config/chainConfig.rs"]
mod chainConfig;

use chainConfig::MOVEMENT_DEVNET_CHAIN_ID;
use alloy::primitives::{Address, U256};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub market_type: Option<String>,
    pub symbol: String,
    pub asset_symbol: Option<String>,
    pub base_symbol: Option<String>,
    pub decimals: u8,
    pub address: Address,
    pub price_decimals: Option<u8>,
    pub wrapped_address: Option<Address>,
    pub coingecko_url: Option<String>,
    pub coingecko_symbol: Option<String>,
    pub metamask_symbol: Option<String>,
    pub explorer_symbol: Option<String>,
    pub explorer_url: Option<String>,
    pub reserves_url: Option<String>,
    pub image_url: Option<String>,
    
    pub is_usdg: bool,
    pub is_native: bool,
    pub is_wrapped: bool,
    pub is_shortable: bool,
    pub is_stable: bool,
    pub is_synthetic: bool,
    pub is_temp_hidden: bool,
    pub is_chart_disabled: bool,
    pub is_v1_available: bool,
    pub is_platform_token: bool,
    pub is_platform_trading_token: bool,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            name: String::new(),
            market_type: None,
            symbol: String::new(),
            asset_symbol: None,
            base_symbol: None,
            decimals: 18,
            address: Address::ZERO,
            price_decimals: None,
            wrapped_address: None,
            coingecko_url: None,
            coingecko_symbol: None,
            metamask_symbol: None,
            explorer_symbol: None,
            explorer_url: None,
            reserves_url: None,
            image_url: None,
            is_usdg: false,
            is_native: false,
            is_wrapped: false,
            is_shortable: false,
            is_stable: false,
            is_synthetic: false,
            is_temp_hidden: false,
            is_chart_disabled: false,
            is_v1_available: false,
            is_platform_token: false,
            is_platform_trading_token: false,
        }
    }
}

pub static TOKENS: Lazy<HashMap<u32, Vec<Token>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    map.insert(MOVEMENT_DEVNET_CHAIN_ID, vec![
        // Native token
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
        
        // Wrapped Move
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
        
        // Wrapped Bitcoin
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
        
        // USD Coin
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
        
        // Wrapped Ether
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
        
        // Wrapped Staked Ether
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
        
        // Milkway Staked TIA
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
        
        // Staked Move
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
        
        // GoGoPool AVAX
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
        }
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
