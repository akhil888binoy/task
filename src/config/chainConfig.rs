use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::eth::BlockNumberOrTag;
use alloy::transports::http::Http;
use alloy::network::Network;
use serde::{Serialize, Deserialize};

pub const MOVEMENT_DEVNET_CHAIN_ID: u32 = 30732;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementDevnetConfig;

impl Network for MovementDevnetConfig {
    const NAME: &'static str = "Movement Devnet";
    const CHAIN_ID: u64 = MOVEMENT_DEVNET_CHAIN_ID;
    const NATIVE_CURRENCY_DECIMALS: u8 = 18;
    const NATIVE_CURRENCY_NAME: &'static str = "Move";
    const NATIVE_CURRENCY_SYMBOL: &'static str = "MOVE";
}

pub fn create_movement_provider() -> impl Provider<Http<reqwest::Client>> {
    let rpc_url = "https://mevm.devnet.imola.movementlabs.xyz";
    ProviderBuilder::new()
        .on_http(rpc_url.parse().unwrap())
        .build()
}

pub struct MulticallConfig {
    pub address: alloy::primitives::Address,
    pub block_created: u64,
}

pub fn get_multicall_config() -> MulticallConfig {
    MulticallConfig {
        address: "0xa21B31946003EEC92550bE2180BE0b1A04B40ff3"
            .parse()
            .unwrap(),
        block_created: 5882,
    }
}

