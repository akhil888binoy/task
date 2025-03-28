use alloy::{
    primitives::{address, Address},
    providers::ProviderBuilder,
    sol, transports::http::reqwest::dns::Name,
};
use eyre::Result;
use std::path::Path;
use alloy::primitives::U256;
use crate::{config::chainConfig::{create_bitlayer_provider, BITLAYER}, utils::multicall};
use crate::config::marketConfig::{SINGLE_MARKET_LIST, SINGLE_MARKET_PROPS_DATA};
use crate::utils::getContracts::get_contract;
use crate::utils::hash::{hash_data, hash_string};
use crate::utils::helpers::{BN_ONE, convert_token_address, get_by_key, get_contract_market_prices, serialize_bigint};
use crate::config::datastoreConfig::{borrowing_exponent_factor_key , borrowing_factor_key, claimable_funding_amount_key, funding_decrease_factor_per_second_key, funding_exponent_factor_key, funding_factor_key, funding_increase_factor_per_second_key, is_market_disabled_key, MAX_PNL_FACTOR_FOR_TRADERS_KEY, max_funding_factor_per_second_key, max_open_interest_key, max_pnl_factor_key, max_pool_amount_for_deposit_key, max_pool_amount_key, max_position_impact_factor_for_liquidations_key, max_position_impact_factor_key, min_collateral_factor_for_open_interest, min_collateral_factor_key, min_funding_factor_per_second_key, min_position_impact_pool_amount_key, open_interest_in_tokens_key, open_interest_key, open_interest_reserve_factor_key, pool_amount_adjustment_key, pool_amount_key, position_fee_factor_key, position_impact_exponent_factor_key, position_impact_factor_key, position_impact_pool_amount_key, position_impact_pool_distribution_rate_key, reserve_factor_key, swap_fee_factor_key, swap_impact_exponent_factor_key, swap_impact_factor_key, swap_impact_pool_amount_key, threshold_for_decrease_funding_key, threshold_for_stable_funding_key, virtual_market_id_key, virtual_token_id_key};
use crate::lib::useSingleMarket::use_single_markets;
use crate::lib::useTokensDataRequest::use_tokens_data_request;
use crate::lib::useMarkets::get_markets;
use crate::types::types::{TokenData, TokenPrices};
use alloy::{
    dyn_abi::DynSolValue,
    sol_types::JsonAbiExt,
};
use alloy_multicall::Multicall;
use std::collections::HashMap;
use alloy::{
    sol_types::JsonAbiExt as _, 
};
use DataStore::DataStoreInstance; 
use SingleMarketGlobalReader::SingleMarketGlobalReaderInstance;
use alloy::{
    providers::{CallItemBuilder, Failure, Provider},
};


sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    DataStore,
    "/home/akhil888binoy/developer/taskassignment/rust-pool-stats/src/utils/abi/DataStore.json"
);


sol!(
    #[sol(rpc)]
    SingleMarketGlobalReader,
    "/home/akhil888binoy/developer/taskassignment/rust-pool-stats/src/utils/abi/SingleMarket.json"  // Path to your ABI file
);


#[derive(Debug, Clone)]
pub struct NextFunding {
    pub fundingFactorPerSecond: U256,
    pub  longsPayShorts: bool
}
#[derive(Debug, Clone)]
pub struct VirtualInventory{
    pub virtualPoolAmountForLongToken: U256,
    pub virtualPoolAmountForShortToken: U256,
    pub virtualInventoryForPositions: U256,
}
#[derive(Debug, Clone)]
pub struct MarketInfoResult{
    pub borrowingFactorPerSecondForLongs : U256,
    pub borrowingFactorPerSecondForShorts: U256,
    pub nextFunding: NextFunding,
    pub virtualInventory: VirtualInventory,
    pub isDisabled:bool
}
#[derive(Debug, Clone)]
pub struct PoolValueInfo{
    pub poolValue:U256,
    pub longPnl:U256,
    pub shortPnl:U256,
    pub netPnl:U256,
    pub totalBorrowingFees : U256
}
#[derive(Debug, Clone)]
pub struct MarketProps{
    pub marketToken: Address,
    pub indexToken: Address,
    pub longToken:  Address,
    pub shortToken: Address
}
#[derive(Debug, Clone)]
pub struct MarketPropsData{
    pub longToken:  TokenData,
    pub shortToken: TokenData,
    pub indexToken: TokenData,
    pub name : String
}




pub async fn getSingleDynamic(chain_id: u32) {
    let pass = true;
    let mut marketPropsData = HashMap::new();

    let dataStoreAddress = match get_contract(chain_id, "DataStore") {
        Ok(data)=>data,
        Err(e)=> panic!("Cannot get contract :{}", e )
    };
    let multicallAddress= match get_contract(chain_id, "Multicall"){
        Ok(data)=>data,
        Err(e)=> panic!("Cannot get contract :{}", e )
    };

    let SingleMarketGlobalReaderAddress = match get_contract(chain_id, "SingleMarketGlobalReader"){
        Ok(data)=>data,
        Err(e)=>panic!("Cannot get contract :{}", e )
    };

    let marketlist = match get_markets().await{
        Ok(data)=> data ,
        Err(e)=>panic!("Cannot get markets : {}",e)
    };
    let singleMarkets = marketlist.single_markets;
    let dynamicMarkets = marketlist.dynamic_markets;

    let account : Address = "0xE3D181F5C4672fdCbd2e9Cf021DF95ecFE6DC4A4".parse().unwrap();

    let marketsAddrData = match use_single_markets(chain_id).await{
        Ok(data)=>data,
        Err(e)=>panic!("Cannot fetch data from use single market :{}",e)
    };

    let  marketsData = marketsAddrData.0;
    let marketsAddresses = marketsAddrData.1;

    let tokensData = match use_tokens_data_request(chain_id).await{
        Ok(data)=> match data.tokens_data {
            Some(data)=>data,
            None=>panic!("Not token data for this chainid")
        },
        Err(e)=>panic!("Cannot send data request : {}",e)
    };

    let mut tokenMap = HashMap::new();
    for (token_address, token_data) in tokensData.iter() {
        tokenMap.insert(token_address.to_string().to_lowercase(), token_data.clone()); 
    };


    for (marketaddress , market) in marketsData.iter(){

        let marketTokenAddressM = &market.market_token_address;
        let indexTokendress = &market.index_token_address;
        let marketLongTokenAddress= &market.long_token_address;
        let marketShortTokenAddress= &market.short_token_address;
        let name = &market.name;
        let isSameCollateralsdata= &market.is_same_collaterals;

        if (marketTokenAddressM.is_empty() || indexTokendress.is_empty() || marketLongTokenAddress.is_empty() || marketShortTokenAddress.is_empty()) {
            panic!("Invalid or missing address fields in market data:{:?}", market);
            continue; 
        }
        
        let accountAddress = format!("{}-{}",marketTokenAddressM , indexTokendress);
         
       if  singleMarkets.is_empty(){
            panic!("No single market addresses found")
       }
        
        if !singleMarkets.contains(marketTokenAddressM){
            panic!("Market not found : {:?}", marketTokenAddressM);
        }
        let accAddr : Address = accountAddress.parse().unwrap();
        let market = match get_by_key(&marketsData, &accAddr){
            Some(data)=>data,
            None=>panic!("Cannot get market data for this address :{}", accAddr)
        };

        let marketPrices = match get_contract_market_prices(&tokensData, &market){
            Some(data)=>data,
            None=>panic!("Cannot get market prices")
        };


        let indexToken = match tokenMap.get(&indexTokendress.to_string().to_lowercase()){
            Some(data)=>data,
            None => panic!("Cannot get index token from tokensMap")
        };

        let longToken = match tokenMap.get(&marketLongTokenAddress.to_string().to_lowercase()){
            Some(data)=>data,
            None=>panic!("Long token not found in tokenMap")
        };

        let shortToken = match tokenMap.get(&marketShortTokenAddress.to_string().to_lowercase()){
            Some(data)=>data,
            None=>panic!("short token not found in tokenMap")
        };

        let marketProps= MarketProps{
                marketToken: market.market_token_address,
                indexToken: market.index_token_address,
                longToken: market.long_token_address,
                shortToken: market.short_token_address,
        };
        
        marketPropsData.insert(marketProps.marketToken, MarketPropsData{
            longToken:longToken.clone(),
            shortToken:shortToken.clone(),
            indexToken:indexToken.clone(),
            name: name.clone()
        });

        
        let provider = create_bitlayer_provider();
        
        let mut multicall = Multicall::with_provider_chain_id(&provider).await.unwrap();

        let  data_store_address= match get_contract(BITLAYER, "DataStore"){
            Ok(data)=>data,
            Err(e)=> panic!("Error DataStore not found")
        };

        let  single_market_reader_address= match get_contract(BITLAYER, "SingleMarketGlobalReader"){
            Ok(data)=>data,
            Err(e)=> panic!("Error DataStore not found")
        };
        let  multicall_address= match get_contract(BITLAYER, "Multicall"){
            Ok(data)=>data,
            Err(e)=> panic!("Error DataStore not found")
        };

        let data_store = DataStoreInstance::new(data_store_address, &provider); 
        let market_reader = SingleMarketGlobalReaderInstance::new(
            single_market_reader_address, 
            &provider
        );
    
        let multicall = provider.multicall()
            .add(data_store.getBool(is_market_disabled_key(marketTokenAddressM.clone())))
            .add(data_store.getUint(pool_amount_key(marketTokenAddressM.clone(), marketLongTokenAddress.clone())))
            .add(data_store.getUint(pool_amount_key(marketTokenAddressM.clone(), marketShortTokenAddress.clone())))
            .add(data_store.getUint(max_pool_amount_key(marketTokenAddressM.clone(), marketLongTokenAddress.clone())))
            .add(data_store.getUint(max_pool_amount_key(marketTokenAddressM.clone(), marketShortTokenAddress.clone())))
            .add(data_store.getUint(max_pool_amount_for_deposit_key(marketTokenAddressM.clone(), marketLongTokenAddress.clone())))
            .add(data_store.getUint(max_pool_amount_for_deposit_key(marketTokenAddressM.clone(), marketShortTokenAddress.clone())))
            .add(data_store.getUint(pool_amount_adjustment_key(marketTokenAddressM.clone(), marketLongTokenAddress.clone())))
            .add(data_store.getUint(pool_amount_adjustment_key(marketTokenAddressM.clone(), marketShortTokenAddress.clone())))
            .add(data_store.getUint(reserve_factor_key(marketTokenAddressM.clone(), true)))
            .add(data_store.getUint(reserve_factor_key(marketTokenAddressM.clone(), false)))
            .add(data_store.getUint(open_interest_reserve_factor_key(marketTokenAddressM.clone(), true)))
            .add(data_store.getUint(open_interest_reserve_factor_key(marketTokenAddressM.clone(), false)))
            .add(data_store.getUint(max_open_interest_key(marketTokenAddressM.clone(), true)))
            .add(data_store.getUint(max_open_interest_key(marketTokenAddressM.clone(), false)))
            .add(data_store.getUint(position_impact_pool_amount_key(marketTokenAddressM.clone())));

let multicall1 = provider.multicall()
    .add(data_store.getUint(min_position_impact_pool_amount_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(position_impact_pool_distribution_rate_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(swap_impact_pool_amount_key(marketTokenAddressM.clone(), marketLongTokenAddress.clone())))
    .add(data_store.getUint(swap_impact_pool_amount_key(marketTokenAddressM.clone(), marketShortTokenAddress.clone())))
    .add(data_store.getUint(borrowing_factor_key(marketTokenAddressM.clone(), true)))
    .add(data_store.getUint(borrowing_factor_key(marketTokenAddressM.clone(), false)))
    .add(data_store.getUint(borrowing_exponent_factor_key(marketTokenAddressM.clone(), true)))
    .add(data_store.getUint(borrowing_exponent_factor_key(marketTokenAddressM.clone(), false)))
    .add(data_store.getUint(funding_factor_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(funding_exponent_factor_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(funding_increase_factor_per_second_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(funding_decrease_factor_per_second_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(threshold_for_stable_funding_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(threshold_for_decrease_funding_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(min_funding_factor_per_second_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(max_funding_factor_per_second_key(marketTokenAddressM.clone())));

    let multicall2 = provider.multicall()
    .add(data_store.getUint(max_pnl_factor_key(MAX_PNL_FACTOR_FOR_TRADERS_KEY.clone(), marketTokenAddressM.clone(), true)))
    .add(data_store.getUint(max_pnl_factor_key(MAX_PNL_FACTOR_FOR_TRADERS_KEY.clone(), marketTokenAddressM.clone(), false)))
    .add(data_store.getUint(claimable_funding_amount_key(marketTokenAddressM.clone(), marketLongTokenAddress.clone(), account.clone())))
    .add(data_store.getUint(claimable_funding_amount_key(marketTokenAddressM.clone(), marketShortTokenAddress.clone(), account.clone())))
    .add(data_store.getUint(position_fee_factor_key(marketTokenAddressM.clone(), true)))
    .add(data_store.getUint(position_fee_factor_key(marketTokenAddressM.clone(), false)))
    .add(data_store.getUint(position_impact_factor_key(marketTokenAddressM.clone(), true)))
    .add(data_store.getUint(position_impact_factor_key(marketTokenAddressM.clone(), false)))
    .add(data_store.getUint(max_position_impact_factor_key(marketTokenAddressM.clone(), true)))
    .add(data_store.getUint(max_position_impact_factor_key(marketTokenAddressM.clone(), false)))
    .add(data_store.getUint(max_position_impact_factor_for_liquidations_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(min_collateral_factor_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(min_collateral_factor_for_open_interest(marketTokenAddressM.clone(), true)))
    .add(data_store.getUint(min_collateral_factor_for_open_interest(marketTokenAddressM.clone(), false)))
    .add(data_store.getUint(position_impact_exponent_factor_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(swap_fee_factor_key(marketTokenAddressM.clone(), true)));

    let multicall3 = provider.multicall()
    .add(data_store.getUint(swap_fee_factor_key(marketTokenAddressM.clone(), false)))
    .add(data_store.getUint(swap_impact_factor_key(marketTokenAddressM.clone(), true)))
    .add(data_store.getUint(swap_impact_factor_key(marketTokenAddressM.clone(), false)))
    .add(data_store.getUint(swap_impact_exponent_factor_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(open_interest_key(marketTokenAddressM.clone(), marketLongTokenAddress.clone(), true)))
    .add(data_store.getUint(open_interest_key(marketTokenAddressM.clone(), marketShortTokenAddress.clone(), true)))
    .add(data_store.getUint(open_interest_key(marketTokenAddressM.clone(), marketLongTokenAddress.clone(), false)))
    .add(data_store.getUint(open_interest_key(marketTokenAddressM.clone(), marketShortTokenAddress.clone(), false)))
    .add(data_store.getUint(open_interest_in_tokens_key(marketTokenAddressM.clone(), marketLongTokenAddress.clone(), true)))
    .add(data_store.getUint(open_interest_in_tokens_key(marketTokenAddressM.clone(), marketShortTokenAddress.clone(), true)))
    .add(data_store.getUint(open_interest_in_tokens_key(marketTokenAddressM.clone(), marketLongTokenAddress.clone(), false)))
    .add(data_store.getUint(open_interest_in_tokens_key(marketTokenAddressM.clone(), marketShortTokenAddress.clone(), false)))
    .add(data_store.getUint(virtual_market_id_key(marketTokenAddressM.clone())))
    .add(data_store.getUint(virtual_token_id_key(marketLongTokenAddress.clone())))
    .add(data_store.getUint(virtual_token_id_key(marketShortTokenAddress.clone())));
    
    let single_market_prices = SingleMarketUtils::SingleMarketPrices {
        indexTokenPrice: Price::Props {
            min: marketPrices.index_token_price.min_price,
            max: marketPrices.index_token_price.max_price,
        },
        longTokenPrice: Price::Props {
            min: marketPrices.long_token_price.min_price,
            max: marketPrices.long_token_price.max_price,
        },
        shortTokenPrice: Price::Props {
            min: marketPrices.short_token_price.min_price,
            max: marketPrices.short_token_price.max_price,
        },
    };

    let multicall4 = provider.multicall()
    .add(market_reader.getSingleMarketInfo(
        data_store_address,
        single_market_prices, 
        market.market_token_address
    ))
    .add(market_reader.getSingleMarketTokenPrice(
        data_store_address,
        SingleMarket::Props {
            marketToken: market.market_token_address,
            indexToken: market.index_token_address,
            longToken: market.long_token_address,
            shortToken: market.short_token_address,
        },
        Price::Props {
            min: marketPrices.index_token_price.min_price,
            max: marketPrices.index_token_price.max_price,
        },
        Price::Props {
            min: marketPrices.long_token_price.min_price,
            max: marketPrices.long_token_price.max_price,
        },
        Price::Props {
            min: marketPrices.short_token_price.min_price,
            max: marketPrices.short_token_price.max_price,
        },
        *MAX_PNL_FACTOR_FOR_TRADERS_KEY,
        true
    ))
    .add(market_reader.getSingleMarketTokenPrice(
        data_store_address,
        SingleMarket::Props {
            marketToken: market.market_token_address,
            indexToken: market.index_token_address,
            longToken: market.long_token_address,
            shortToken: market.short_token_address,
        },
        Price::Props {
            min: marketPrices.index_token_price.min_price,
            max: marketPrices.index_token_price.max_price,
        },
        Price::Props {
            min: marketPrices.long_token_price.min_price,
            max: marketPrices.long_token_price.max_price,
        },
        Price::Props {
            min: marketPrices.short_token_price.min_price,
            max: marketPrices.short_token_price.max_price,
        },
        *MAX_PNL_FACTOR_FOR_TRADERS_KEY,
        true
    ));
    
    // Execute all multicalls sequentially
let (
    isDisabled,
    longPoolAmount,
    shortPoolAmount,
    maxLongPoolAmount,
    maxShortPoolAmount,
    maxLongPoolAmountForDeposit,
    maxShortPoolAmountForDeposit,
    longPoolAmountAdjustment,
    shortPoolAmountAdjustment,
    reserveFactorLong,
    reserveFactorShort,
    openInterestReserveFactorLong,
    openInterestReserveFactorShort,
    maxOpenInterestLong,
    maxOpenInterestShort,
    positionImpactPoolAmount,
) = match multicall.aggregate().await {
    Ok(data)=> data,
    Err(e)=>panic!("multicall 0 error")
};

let (
    minPositionImpactPoolAmount,
    positionImpactPoolDistributionRate,
    swapImpactPoolAmountLong,
    swapImpactPoolAmountShort,
    borrowingFactorLong,
    borrowingFactorShort,
    borrowingExponentFactorLong,
    borrowingExponentFactorShort,
    fundingFactor,
    fundingExponentFactor,
    fundingIncreaseFactorPerSecond,
    fundingDecreaseFactorPerSecond,
    thresholdForStableFunding,
    thresholdForDecreaseFunding,
    minFundingFactorPerSecond,
    maxFundingFactorPerSecond,
) = match multicall1.aggregate().await{
    Ok(data)=>data,
    Err(e)=>panic!("Error multicall 1 ")
};

let (
    maxPnlFactorForTradersLong,
    maxPnlFactorForTradersShort,
    claimableFundingAmountLong,
    claimableFundingAmountShort,
    positionFeeFactorForPositiveImpact,
    positionFeeFactorForNegativeImpact,
    positionImpactFactorPositive,
    positionImpactFactorNegative,
    maxPositionImpactFactorPositive,
    maxPositionImpactFactorNegative,
    maxPositionImpactFactorForLiquidations,
    minCollateralFactor,
    minCollateralFactorForOpenInterestLong,
    minCollateralFactorForOpenInterestShort,
    positionImpactExponentFactor,
    swapFeeFactorForPositiveImpact,
) = match multicall2.aggregate().await{
    Ok(data)=>data,
    Err(e)=>panic!("Multicall 2 error")
};

let (
    swapFeeFactorForNegativeImpact,
    swapImpactFactorPositive,
    swapImpactFactorNegative,
    swapImpactExponentFactor,
    longInterestUsingLongToken,
    longInterestUsingShortToken,
    shortInterestUsingLongToken,
    shortInterestUsingShortToken,
    longInterestInTokensUsingLongToken,
    longInterestInTokensUsingShortToken,
    shortInterestInTokensUsingLongToken,
    shortInterestInTokensUsingShortToken,
    virtualMarketId,
    virtualLongTokenId,
    virtualShortTokenId,
) = match multicall3.aggregate().await{
    Ok(data)=>data,
    Err(e)=> panic!("Error multicall 3")
};

let (
    marketInfo,
    marketTokenPriceMin,
    marketTokenPriceMax,
) = match multicall4.aggregate().await{
    Ok(data)=>data,
    Err(e)=>panic!("Error in multicall 4")
};


    let marketDivisor = if isSameCollateralsdata.clone() { U256::from(2) } else { *BN_ONE };
    let longInterestUsingLongTokenNew =longInterestUsingLongToken._0; 
    let longInterestUsingShortTokenNew = longInterestUsingShortToken._0;
    let shortInterestUsingLongTokenNew = shortInterestUsingLongToken._0;
    let shortInterestUsingShortTokenNew = shortInterestUsingShortToken._0;  





    }

    




}