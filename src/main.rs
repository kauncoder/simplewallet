mod wallet;
use wallet::{get_wallet_details, Wallet};
#[tokio::main]
async fn main() {
    let list_wallet_name = vec![
        "wallet_name_one",
        "wallet_name_two",
        "wallet_name_three",
        "wallet_name_four",
    ];
    //provide new names for wallets
    for wallet_name in list_wallet_name {
        //if file already exists and has info in it, then ignore, else add new wallet info to it
        let wallet = Wallet::build_wallet().unwrap();
        wallet.store_wallet(wallet_name);
        let new_wallet = get_wallet_details("wallet.json");
        println!("{:?}", new_wallet);
    }
}
