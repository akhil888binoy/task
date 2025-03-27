use alloy::providers::{Provider, ProviderBuilder,RootProvider};
use alloy::rpc::types::eth::BlockNumberOrTag;
use alloy::transports::http::Http;
use alloy::network::Network;
use serde::{Serialize, Deserialize};
use eyre::Result;

use alloy::providers::fillers::{FillProvider , JoinFill , GasFiller,NonceFiller,ChainIdFiller,BlobGasFiller};
use alloy::providers::Identity;

pub const BITLAYER: u32 = 200810;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementDevnetConfig;

// impl Network for MovementDevnetConfig {
//     const NAME: &'static str = "Movement Devnet";
//     const CHAIN_ID: u64 = MOVEMENT_DEVNET_CHAIN_ID;
//     const NATIVE_CURRENCY_DECIMALS: u8 = 18;
//     const NATIVE_CURRENCY_NAME: &'static str = "Move";
//     const NATIVE_CURRENCY_SYMBOL: &'static str = "MOVE";
// }

pub fn create_bitlayer_provider() -> FillProvider<JoinFill<Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>, RootProvider> {
    let rpc_url = "https://testnet-rpc.bitlayer.org"
        .parse()
        .expect("Invalid RPC URL");
    let provider = ProviderBuilder::new().on_http(rpc_url);
    provider
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

