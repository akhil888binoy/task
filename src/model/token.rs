// src/models/token.rs
use alloy::primitives::{Address, U256};
use serde::{Serialize, Deserialize};

/// Unified Token type combining all fields from both definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    // Core Identification
    pub name: String,
    pub symbol: String,
    pub address: Address,
    pub decimals: u32,  // Standard Ethereum practice (u8)

    // Extended Metadata
    #[serde(default)]
    pub market_type: Option<String>,
    #[serde(default)]
    pub asset_symbol: Option<String>,
    #[serde(default)]
    pub base_symbol: Option<String>,
    
    // Pricing Information
    #[serde(default)]
    pub price_decimals: Option<u8>,
    #[serde(default)]
    pub wrapped_address: Option<Address>,

    // External References
    #[serde(default)]
    pub coingecko_url: Option<String>,
    #[serde(default)]
    pub coingecko_symbol: Option<String>,
    #[serde(default)]
    pub metamask_symbol: Option<String>,
    #[serde(default)]
    pub explorer_symbol: Option<String>,
    #[serde(default)]
    pub explorer_url: Option<String>,
    #[serde(default)]
    pub reserves_url: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,

    // Token Classification Flags
    #[serde(default)]
    pub is_usdg: bool,
    #[serde(default)]
    pub is_native: bool,
    #[serde(default)]
    pub is_wrapped: bool,
    #[serde(default)]
    pub is_shortable: bool,
    #[serde(default)]
    pub is_stable: bool,
    #[serde(default)]
    pub is_synthetic: bool,
    
    // UI/Behavior Flags
    #[serde(default)]
    pub is_temp_hidden: bool,
    #[serde(default)]
    pub is_chart_disabled: bool,
    
    // Versioning
    #[serde(default)]
    pub is_v1_available: bool,
    
    // Platform-Specific
    #[serde(default)]
    pub is_platform_token: bool,
    #[serde(default)]
    pub is_platform_trading_token: bool,

    // New fields from types.rs
    #[serde(default)]
    pub min_price: Option<U256>,
    #[serde(default)]
    pub max_price: Option<U256>,
    #[serde(default)]
    pub balance: Option<U256>,
    #[serde(default)]
    pub total_supply: Option<U256>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            name: String::new(),
            symbol: String::new(),
            address: Address::ZERO,
            decimals: 18,
            
            // Metadata
            market_type: None,
            asset_symbol: None,
            base_symbol: None,
            
            // Pricing
            price_decimals: None,
            wrapped_address: None,
            
            // External
            coingecko_url: None,
            coingecko_symbol: None,
            metamask_symbol: None,
            explorer_symbol: None,
            explorer_url: None,
            reserves_url: None,
            image_url: None,
            
            // Flags
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
            
            // New numeric fields
            min_price: None,
            max_price: None,
            balance: None,
            total_supply: None,
        }
    }
}

// Additional helper methods
impl Token {
    /// Returns the effective price decimals (defaults to 18 if not specified)
    pub fn effective_price_decimals(&self) -> u8 {
        self.price_decimals.unwrap_or(18)
    }

    /// Checks if token has price data
    pub fn has_price_data(&self) -> bool {
        self.min_price.is_some() || self.max_price.is_some()
    }
}