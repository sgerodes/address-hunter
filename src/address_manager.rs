#[warn(unused_imports)]

use bip39::{Mnemonic, Language, MnemonicType, Seed};
use tiny_hderive::bip32::ExtendedPrivKey;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Digest, Keccak256};
use hex;
use std::fmt;
use libsecp256k1::{SecretKey, PublicKey, Error, Message, Signature, RecoveryId};
use rand::RngCore;


pub struct KeyBundle {
    pub public_address_no_0x: String,
    pub mnemonic: String,
    pub private_key: String,
}
impl KeyBundle {
    pub fn new(mnemonic: String, private_key: String, public_address: String) -> Self {
        KeyBundle {
            public_address_no_0x: public_address,
            mnemonic,
            private_key,
        }
    }
}

impl fmt::Debug for KeyBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyBundle")
            .field("public_address", &self.public_address_no_0x)
            .finish()
    }
}

impl fmt::Display for KeyBundle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyBundle(address:{})", 
            self.public_address_no_0x)
    }
}

pub fn create_key_bundle() -> KeyBundle {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let (private_key, public_address) = create_eth_address(mnemonic.phrase());
    //let key_bundle = KeyBundle::new(mnemonic.phrase().to_owned(), private_key, public_address);
    //println!("{}", key_bundle);
    return KeyBundle::new(mnemonic.phrase().to_owned(), private_key, public_address);
}

pub fn create_eth_address_2() -> Result<(String, String), Error> {
    // Generate a new random private key
    let mut rng = rand::thread_rng();
    let mut secret_key = [0u8; 32];
    rng.fill_bytes(&mut secret_key);

    let secret_key = SecretKey::parse(&secret_key)?;
    let public_key = PublicKey::from_secret_key(&secret_key);

    // Hash the public key to create the address
    let mut hasher = Keccak256::new();
    hasher.update(&public_key.serialize()[1..]);
    let result = hasher.finalize();

    // Ethereum address is the last 20 bytes of the hash
    let address = &result.as_slice()[12..];

    // Convert to hex string
    let address_no_0x = format!("{}", hex::encode(address));

    // Convert private key to hex string
    let private_key_hex = hex::encode(secret_key.serialize());

    Ok((private_key_hex, address_no_0x))
}




pub fn create_eth_address(mnemonic: &str) -> (String, String) {
    // Generate the Seed from the Mnemonic
    let seed = Seed::new(&Mnemonic::from_phrase(mnemonic, Language::English).unwrap(), "");

    // Create the BIP32 Extended Private Key
    let ext_priv_key = ExtendedPrivKey::derive(seed.as_bytes(), "m/44'/60'/0'/0/0").unwrap();

    // Create the Secp256k1 secret key
    let secret_key = SecretKey::from_slice(&ext_priv_key.secret()).unwrap();

    // Create the Secp256k1 public key
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Hash the public key to create the address
    let mut hasher = Keccak256::new();
    hasher.update(&public_key.serialize()[1..]);
    let result = hasher.finalize();

    // Ethereum address is the last 20 bytes of the hash
    let address = &result.as_slice()[12..];

    // Convert to hex string
    // let address_hex = format!("0x{}", hex::encode(address));
    // Convert to hex string
    let address_no_0x = format!("{}", hex::encode(address));

    // Convert private key to hex string
    let private_key_hex = hex::encode(ext_priv_key.secret());

    (private_key_hex, address_no_0x)
}
