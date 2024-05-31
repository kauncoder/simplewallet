//this will create the wallet  and add them to the web3 node (ETH node)
use anyhow::Result;
use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;
use secp256k1::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::{fs::OpenOptions, io::BufReader};
use web3::signing::keccak256;
use web3::types::{Address, H160};
// pub struct Keys {
//     secret_key: SecretKey,
//     public_key: PublicKey,
// // }
// pub struct Wallet {
//     pub secret_key: SecretKey,
//     pub public_key: PublicKey,
//     pub public_address: H160,
//     pub recovery_phrase: Option<String>,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
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
                //build keys from seed phrase
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
        self
    }
    fn build(&mut self) -> Result<Wallet> {
        Ok(Wallet {
            secret_key: format!("{:?}", self.secret_key.unwrap()),
            public_key: format!("{:?}", self.public_key.unwrap()),
            public_address: format!("{:?}", self.public_address.unwrap()),
            recovery_phrase: self.recovery_phrase.clone(),
        })
    }
}

impl Wallet {
    pub fn build_wallet() -> Result<Wallet> {
        return WalletBuilder::new()
            .recovery_phrase()
            .keys()
            .address()
            .build();
    }

    pub fn get_public_address(self) {
        println!("{:?}", self.public_address);
    }

    pub fn store_wallet(&self, file_path: &str) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self).unwrap();
    }
}

pub fn get_wallet_details(file_path: &str) -> Result<Wallet> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let buf_reader = BufReader::new(file);
    let wallet: Wallet = serde_json::from_reader(buf_reader)?;
    Ok(wallet)
}
