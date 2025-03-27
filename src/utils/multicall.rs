// use alloy::{
//     providers::Provider,
//     sol,
//     sol_types::SolCall,
//     transports::Transport,
//     primitives::{Address, Bytes},
// };
// use async_trait::async_trait;
// use std::{collections::HashMap, error::Error, fmt};
// use thiserror::Error;


// use crate::types::types::{MulticallRequestConfig, MulticallResult};
// use crate::utils::getContracts::get_contract;

// // Define the Multicall ABI
// sol! {
//     interface Multicall {
//         function aggregate((address,bytes)[] memory calls) external view returns (uint256 blockNumber, bytes[] memory returnData);
//     }
// }

// #[derive(Error, Debug)]
// pub enum MulticallError {
//     #[error("No Multicall contract found for chainId {0}")]
//     NoMulticallContract(u32),
//     #[error("No valid calls provided for multicall")]
//     NoValidCalls,
//     #[error("Contract error: {0}")]
//     ContractError(#[from] alloy::contract::Error),
//     #[error("ABI encoding error: {0}")]
//     AbiError(String),
// }

// pub async fn execute_multicall<T, P, C>(
//     chain_id: u32,
//     provider: &P,
//     request_config: &MulticallRequestConfig<C>,
// ) -> Result<MulticallResult, MulticallError>
// where
//     T: Transport + Clone,
//     P: Provider,
//     C: SolCall,
// {
//     // Get Multicall contract address
//     let multicall_address = get_contract(chain_id, "Multicall")
//         .map_err(|_| MulticallError::NoMulticallContract(chain_id))?;

//     // Prepare calls
//     let mut calls = Vec::new();
//     let mut call_map = HashMap::new();

//     for (request_key, config) in request_config {
//         let contract_address = config.contract_address;
//         let mut contract_calls = Vec::new();

//         for (call_key, call_config) in &config.calls {
//             let call_data = call_config.encode().map_err(|e| {
//                 MulticallError::AbiError(format!("Failed to encode call {}: {}", call_key, e))
//             })?;

//             calls.push((contract_address, Bytes::from(call_data)));
//             contract_calls.push(call_key.clone());
//         }

//         call_map.insert(request_key.clone(), contract_calls);
//     }

//     if calls.is_empty() {
//         return Err(MulticallError::NoValidCalls);
//     }

//     // Create Multicall contract instance
//     let multicall = Multicall::new(multicall_address, provider);

//     // Execute multicall
//     let (_, return_data) = multicall.aggregate(calls).call().await?;

//     // Decode results
//     let mut results = HashMap::new();
//     let mut data_index = 0;

//     for (request_key, call_keys) in call_map {
//         let mut contract_results = HashMap::new();

//         for call_key in call_keys {
//             if data_index >= return_data.len() {
//                 return Err(MulticallError::AbiError("Not enough return data".into()));
//             }

//             let data = &return_data[data_index];
//             let decoded = C::decode(&data, true)
//                 .map_err(|e| MulticallError::AbiError(format!("Failed to decode {}: {}", call_key, e)))?;

//             contract_results.insert(call_key, decoded);
//             data_index += 1;
//         }

//         results.insert(request_key, contract_results);
//     }

//     Ok(MulticallResult {
//         success: true,
//         errors: HashMap::new(),
//         data: results,
//     })
// }