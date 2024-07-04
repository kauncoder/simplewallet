mod connection;
mod transactions;
mod wallet;
use anyhow::Result;
use dotenv::dotenv;
use std::str::FromStr;
use web3::types::{Address, U256};

use std::env;
use wallet::{load_wallet, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let testnet_url = env::var("TESTNET_URL").expect("TESTNET not found");

    //this is a vector to keep more wallets for threshold scenario
    let list_wallet_names = vec!["wallet_name"];

    //create new wallet and store the details in json file
    for wallet_name in &list_wallet_names {
        //if wallet file already exists and has info on it, then ignore, else add new wallet info to it
        match std::path::Path::new(wallet_name).try_exists() {
            Ok(true) => {
                println!("The wallet already exists.");
            }
            Ok(false) => {
                let wallet = Wallet::build_wallet().unwrap();
                wallet.store_wallet(wallet_name);
                println!("New wallet with name {} created", wallet_name);
            }
            Err(_) => {
                println!("Error in reading the path.");
            }
        }
    }
    //get details of the wallet for ETH txns
    let wallet = load_wallet(list_wallet_names[0])?;

    //make connection to the testnet
    let connection = connection::establish_web3_connection(&testnet_url).await?;
    let block_number = connection.eth().block_number().await?;
    println!("ETH block number: {}", &block_number);

    //get wallet amount
    let balance = wallet.get_balance(&connection).await?;
    println!("Balance: {} wei", balance);

    //sign and send
    //pub fn create_eth_transaction(to: Address, wei_value: U256) -> TransactionParameters {
    let test_metamask_addr =
        env::var("TEST_METAMASK_ADDRESS").expect("TEST_METAMASK_ADDRESS not found");
    let to_address = Address::from_str(&test_metamask_addr)?;
    let amount_txn = 500000000000000_i64;
    let tx_params = transactions::set_tx_parameters(to_address, U256::from(amount_txn));
    let secret_key = wallet.get_secret_key()?;
    //convert to web3 signature
    let tx_hash = transactions::sign_and_send(&connection, tx_params, &secret_key).await?;
    println!("Txn hash: {:?}", tx_hash);
    Ok(())
}
