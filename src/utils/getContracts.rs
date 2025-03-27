
use crate::config::chainConfig::BITLAYER;
use alloy::primitives::Address;
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Unknown chainId {0}")]
    UnknownChain(u32),
    #[error("Unknown contract {0} for chainId {1}")]
    UnknownContract(String, u32),
}

pub static CONTRACTS: Lazy<HashMap<u32, HashMap<&'static str, Address>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Bitlayer Testnet contracts
    let bitlayer_contracts = HashMap::from([
        ("Vault", Address::from_str("0xe8966d2374eEF3C145c2363837B36cD1526cE5c1").unwrap()),
        ("Router", Address::from_str("0x0000000000000000000000000000000000000000").unwrap()),
        ("GovToken", Address::from_str("0x71b401A78bCCb2BB205d78363252c26678215544").unwrap()),
        ("USDG", Address::from_str("0x71b401A78bCCb2BB205d78363252c26678215544").unwrap()),
        ("FaucetVault", Address::from_str("0xa9d14143798795Eb3FD4C269C190494664962F8f").unwrap()),
        ("PositionRouter", Address::from_str("0xe8966d2374eEF3C145c2363837B36cD1526cE5c1").unwrap()),
        ("ReferralStorage", Address::from_str("0xC1Ccbb442d03c70a764A39488De4d96651237f7C").unwrap()),
        ("Timelock", Address::from_str("0xb53e832BE370D3D1E425371c42898c9DF70a695F").unwrap()),
        ("DataStore", Address::from_str("0x35C0cc3bFF726CD57451a77df9b74A020bF27E8d").unwrap()),
        ("EventEmitter", Address::from_str("0x2453EC25aE8D8599b6007ae5D0bA409A4EDEA0A0").unwrap()),
        ("SubaccountRouter", Address::from_str("0xD95B31eb19037c546b8933756659486417bE0969").unwrap()),
        ("ExchangeRouter", Address::from_str("0x7707b1ae25cD8Ff6ECE5eDEEB35C39586d680247").unwrap()),
        ("AssetManager", Address::from_str("0x6f0aE25e978E71C543950739db087113640AC945").unwrap()),
        ("MarketManager", Address::from_str("0x50456537e885da724956fF91D8ba79989e782656").unwrap()),
        ("DepositVault", Address::from_str("0x1Ae127d2327415e84E2E3E08FE215C436df6866C").unwrap()),
        ("WithdrawalVault", Address::from_str("0xdca2ACAA341f558799Bce09581266b11979F5474").unwrap()),
        ("OrderVault", Address::from_str("0x251Fe5E88cFF0d81eDDAf4b9dea00fd4D433e3F0").unwrap()),
        ("SyntheticsReader", Address::from_str("0x4BfC74D9bBEDB6F687f147BA6D20b18479964dD4").unwrap()),
        ("SyntheticsRouter", Address::from_str("0xc4a93222E3E5f340bAbE1ADD48C8487D021B844B").unwrap()),
        ("DynamicMarketGlobalReader", Address::from_str("0x0089034403C770387B70CE4Ec14044a38D936b50").unwrap()),
        ("SingleMarketGlobalReader", Address::from_str("0x355a7F3d2E4Bf779b06383D8CcBb459bbEb2863E").unwrap()),
        ("DynamicMarketExchangeRouter", Address::from_str("0x8F75A2c8088dD1ac5eB1114D06b7DC4162A6ab1d").unwrap()),
        ("SingleMarketExchangeRouter", Address::from_str("0xF618d73B0a2eB7882562652481879E00de294F08").unwrap()),
        ("Multicall", Address::from_str("0xc5b265b31986bceF07A17C5c329ee2Fd867f9F7F").unwrap()),
        ("MarketReader", Address::from_str("0x74c9Fe6557fA7790d5477a1193be8f35253979c5").unwrap()),
    ]);

    map.insert(BITLAYER, bitlayer_contracts);
    map
});

pub fn get_contract(chain_id: u32, name: &str) -> Result<Address, ContractError> {
    CONTRACTS
        .get(&chain_id)
        .ok_or(ContractError::UnknownChain(chain_id))?
        .get(name)
        .copied()
        .ok_or_else(|| ContractError::UnknownContract(name.to_string(), chain_id))
}