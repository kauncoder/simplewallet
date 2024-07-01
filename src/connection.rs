//add connection to eth testnet here

//once the connection is made, then what??

//add some transactions logic in transactions file??
use anyhow::Result;

use web3::{
    transports::{self, WebSocket},
    types::{Address, U256},
    Web3,
};
pub async fn establish_web3_connection(url: &str) -> Result<Web3<WebSocket>> {
    let transport = web3::transports::WebSocket::new(url).await?;
    Ok(web3::Web3::new(transport))
}
