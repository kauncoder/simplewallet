mod wallet;
use wallet::Wallet; //, get_address_from_key};
#[tokio::main]
async fn main() {
    let wallet = Wallet::build_wallet();
    wallet.get_public_address();
}
