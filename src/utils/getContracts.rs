#[path = "../config/chainConfig.rs"]
mod chainConfig;

use chainConfig::MOVEMENT_DEVNET_CHAIN_ID;
use alloy::primitives::Address;
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

pub static CONTRACTS: Lazy<HashMap<u32, HashMap<&'static str, Address>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let movement_contracts = HashMap::from([
        ("Vault", "0xe8966d2374eEF3C145c2363837B36cD1526cE5c1".parse().unwrap()),
        ("GovToken", "0x71b401A78bCCb2BB205d78363252c26678215544".parse().unwrap()),
        ("USDG", "0x71b401A78bCCb2BB205d78363252c26678215544".parse().unwrap()),
        ("FaucetVault", "0xF53C0Ba1461e5039628D39C66DB6723786F5D41E".parse().unwrap()),
        ("PositionRouter", "0xe8966d2374eEF3C145c2363837B36cD1526cE5c1".parse().unwrap()),
        ("ReferralStorage", "0x9245999d67c852408978Eee0Df76c3B4E2d83849".parse().unwrap()),
        ("Timelock", "0xb53e832BE370D3D1E425371c42898c9DF70a695F".parse().unwrap()),
        ("DataStore", "0xeAB59d23C59083C0c378106eCac055E809638Da4".parse().unwrap()),
        ("EventEmitter", "0xcd79417B6c3E0Bfe6dD59E116f430e2DBE135AAF".parse().unwrap()),
        ("SubaccountRouter", "0xD95B31eb19037c546b8933756659486417bE0969".parse().unwrap()),
        ("ExchangeRouter", "0x7707b1ae25cD8Ff6ECE5eDEEB35C39586d680247".parse().unwrap()),
        ("AssetManager", "0x602B0D9F0a150C4D69a3a48FBbe25e1c5b5BC564".parse().unwrap()),
        ("MarketManager", "0xD7a31Cde740aB70eA3AfeA6Bb9d83B06dBB3c0ac".parse().unwrap()),
        ("DepositVault", "0x3a71424a4D5169b9220FEa7De7F81385acF74F7e".parse().unwrap()),
        ("WithdrawalVault", "0x2fa82c70912B856251a75ae12572917fA5f73eeE".parse().unwrap()),
        ("OrderVault", "0xDAB7BB57182F395638bB8Cb781ddC9Fc45F247D0".parse().unwrap()),
        ("SyntheticsRouter", "0xEd1A5dAbc1944626Dc91816eacFA6f6557820d4D".parse().unwrap()),
        ("DynamicMarketGlobalReader", "0x88305A88249D3f48b3a0141f7c8580D1dD1B75cF".parse().unwrap()),
        ("SingleMarketGlobalReader", "0x14834c8ff5FAB959Fb3c19467A6EAA3265A0D75A".parse().unwrap()),
        ("DynamicMarketExchangeRouter", "0x4AacB28FC5dEEE1314462a59230CE007f1FC0e73".parse().unwrap()),
        ("SingleMarketExchangeRouter", "0xD06758a08a78ef28f56b4EfA35AC87eF21D56f15".parse().unwrap()),
        ("Multicall", "0xa21B31946003EEC92550bE2180BE0b1A04B40ff3".parse().unwrap()),
        ("MarketReader", "0xDFc9665Acd5f30E6565495E2E2d2b876129BE28A".parse().unwrap()),
    ]);

    map.insert(MOVEMENT_DEVNET_CHAIN_ID, movement_contracts);
    map
});

pub fn get_contract(chain_id: u32, name: &str) -> Result<Address, String> {
    CONTRACTS
        .get(&chain_id)
        .ok_or_else(|| format!("Unknown chainId {}", chain_id))?
        .get(name)
        .cloned()
        .ok_or_else(|| format!("Unknown contract \"{}\" for chainId {}", name, chain_id))
}
