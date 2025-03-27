
pub mod model;
pub mod config;
pub mod lib;
pub mod utils;
pub mod types;


use crate::config::chainConfig::{BITLAYER};
use crate::lib::useSingleMarket::use_single_markets;
use crate::lib::useMarkets::get_markets;

#[tokio::main]
async fn main() {
   // test singleMarket function over here
   get_markets().await;
   use_single_markets(BITLAYER).await;

}
