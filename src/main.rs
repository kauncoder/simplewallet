mod wallet;
use wallet::Wallet;
#[tokio::main]
async fn main() {
    let wallet = Wallet::build_wallet();
    wallet.get_public_address();
}
