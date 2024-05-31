//this will create the connection and add them to the web3 node (ETH node)

use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;
use secp256k1::{PublicKey, SecretKey};
use web3::signing::keccak256;
use web3::types::{Address, H160};

// pub struct Keys {
//     secret_key: SecretKey,
//     public_key: PublicKey,
// }
pub struct Wallet {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub public_address: H160,
    pub recovery_phrase: Option<String>,
}

pub struct WalletBuilder {
    pub secret_key: Option<SecretKey>,
    pub public_key: Option<PublicKey>,
    pub public_address: Option<H160>,
    pub recovery_phrase: Option<String>,
}
impl WalletBuilder {
    fn new() -> WalletBuilder {
        //create with all None
        WalletBuilder {
            secret_key: None,
            public_key: None,
            public_address: None,
            recovery_phrase: None,
        }
    }

    fn recovery_phrase(&mut self) -> &mut Self {
        //add seed phrase to existing values
        self.recovery_phrase = Some("somephrase".to_string());
        self
    }
    fn keys(&mut self) -> &mut Self {
        match &self.recovery_phrase {
            Some(_seed_value) => {
                //build keys from that
                let secp = Secp256k1::new();
                let genval = secp.generate_keypair(&mut OsRng);
                self.secret_key = Some(genval.0);
                self.public_key = Some(genval.1);
            }
            None => {
                //generate some random keys
                let secp = Secp256k1::new();
                let genval = secp.generate_keypair(&mut OsRng);
                self.secret_key = Some(genval.0);
                self.public_key = Some(genval.1);
            }
        }
        self
    }
    fn address(&mut self) -> &mut Self {
        let public_key = self.public_key.unwrap().serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        self.public_address = Some(Address::from_slice(&hash[12..]));

        // let public_key = self.public_key.unwrap().serialize_uncompressed();
        // self.public_address = Some(Address::from_slice(&keccak256(&public_key[1..])));
        self
    }
    fn build(&mut self) -> Wallet {
        Wallet {
            secret_key: self.secret_key.unwrap(),
            public_key: self.public_key.unwrap(),
            public_address: self.public_address.unwrap(),
            recovery_phrase: self.recovery_phrase.clone(),
        }
    }
}

impl Wallet {
    pub fn build_wallet() -> Wallet {
        //calls wallet builder and returns a built wallet; check for errors later
        return WalletBuilder::new()
            .recovery_phrase()
            .keys()
            .address()
            .build();
    }

    pub fn get_public_address(self) {
        println!("{:?}", self.public_address);
    }

    /*
    fn get_public_key(self) {
        println!("{:?}", self.public_key);
    }

    fn get_secret_key(self) {
        println!("{:?}", self.secret_key);
    } //how to make this inaccessible??

    fn get_recovery_phrase(self) {
        println!("{:?}", self.recovery_phrase);
    }
    */
}
