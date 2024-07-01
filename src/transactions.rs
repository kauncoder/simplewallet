//just run transactions logic here
use anyhow::Result;
use web3::{
    signing::SecretKey,
    transports,
    types::{Address, TransactionParameters, H256, U256},
    Web3,
};

//use wallet to send and receive values
pub fn set_tx_parameters(to: Address, wei_value: U256) -> TransactionParameters {
    TransactionParameters {
        to: Some(to),
        value: wei_value,
        ..Default::default()
    }
}

pub async fn sign_and_send(
    web3: &Web3<transports::WebSocket>,
    transaction: TransactionParameters,
    secret_key: &SecretKey,
) -> Result<H256> {
    let signed = web3
        .accounts()
        .sign_transaction(transaction, secret_key)
        .await?;
    println!("was signed");
    let transaction_result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;
    Ok(transaction_result)
}
