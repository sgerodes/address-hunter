pub mod eth_wallet {

    use ethers::core::rand::thread_rng;
    use tiny_keccak::keccak256;
    use web3::types::Address;
    use eth_checksum;

    use secp256k1::{PublicKey, Secp256k1, SecretKey};

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Wallet {
        pub secret_key: String,
        pub public_key: String,
        pub address: String,
        pub address_checksummed: String
    }

    impl Wallet {
        pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
            let addr: Address = public_key_address(&public_key);
            let address: String = hex::encode(addr);
            let address_checksummed: String = checksummed(&address);
            Wallet {
                secret_key: hex::encode(&secret_key.secret_bytes()),
                public_key: public_key.to_string(),
                address: address,
                address_checksummed: address_checksummed
            }
        }
    }

    pub fn generate_random_wallet() -> Wallet {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut thread_rng());
        Wallet::new(&secret_key, &public_key)
    }

    pub fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        // debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    }

    pub fn checksummed(address: &String) -> String {
        eth_checksum::checksum(address)[2..].to_string()
    }

}

pub mod polka_wallet {
    use sp_core::{sr25519, Pair};
    use sp_core::crypto::{Ss58Codec};
    use bip39::{Mnemonic, Language};
    use rand::RngCore;

    #[derive(Debug, Clone)]
    pub struct PolkaWallet {
        pub secret_key: String,
        pub public_key: String,
        pub address: String,
        pub mnemonic: String,
    }
    impl PolkaWallet {
        pub fn new(secret_key: &sr25519::Pair, mnemonic: &str) -> Self {
            let public_key = secret_key.public();
            let address = public_key.to_ss58check();
            PolkaWallet {
                secret_key: hex::encode(secret_key.to_raw_vec()),
                public_key: hex::encode(public_key),
                address,
                mnemonic: mnemonic.to_string(),
            }
        }
    }

    pub fn generate_random_wallet() -> PolkaWallet {
        let mut entropy = [0u8; 16]; // 128 bits for 12-word mnemonic
        rand::thread_rng().fill_bytes(&mut entropy);

        let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
        // Convert mnemonic to seed
        let normalized_passphrase = ""; // Using an empty passphrase
        let seed = mnemonic.to_seed_normalized(normalized_passphrase);

        // Generate key pair from seed slice
        let pair = sr25519::Pair::from_seed_slice(&seed).unwrap();

        // Create and return the wallet
        PolkaWallet::new(&pair, mnemonic.phrase())
    }
}


pub mod eth_wallet_simple {
    // https://ethereum.stackexchange.com/questions/3542/how-are-ethereum-addresses-generated
    
    use rand::Rng;
    use num_bigint::{BigUint, RandBigInt, RandomBits};
    use num_traits::{Zero, One};
    use once_cell::sync::Lazy;
    use lazy_static::lazy_static;

    use ethers::core::rand::thread_rng;
    use tiny_keccak::keccak256;
    use web3::types::Address;

    use secp256k1::{PublicKey, Secp256k1, SecretKey};

    pub fn random_biguint_in_hex64_range() -> BigUint {
        let mut rng = rand::thread_rng();
        rng.sample(RandomBits::new(256))
    }
    
    #[derive(Debug)]
    pub struct Wallet {
        pub secret_key: String,
        pub public_key: String,
        pub address: String,
    }

    impl Wallet {
        pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
            let addr: Address = public_key_address(&public_key);
            Wallet {
                secret_key: hex::encode(&secret_key.secret_bytes()),
                public_key: public_key.to_string(),
                address: hex::encode(addr),
            }
        }
    }

    pub struct Generator {
        pub current_biguint: BigUint,
    }

    impl Generator {
        pub fn new() -> Generator {
            let mut gen = Generator {
                current_biguint: BigUint::one(),
            };
            gen.randomize();
            gen
        }
    
        pub fn increment(&mut self) {
            self.current_biguint += BigUint::one();
        }
    
        pub fn randomize(&mut self) {
            let mut rng = rand::thread_rng();
            self.current_biguint = rng.gen_biguint(128);
        }
    }

    pub fn parse_hex64_padded(bigint: BigUint) -> String {
        format!("{:064x}", bigint)
    }

    pub fn generate_private_key(biguint: BigUint) -> String {
        parse_hex64_padded(biguint)
    }

    pub fn generate_ecdsa_key_pair_from_private_key(biguint: BigUint) -> Result<Wallet, Box<dyn std::error::Error>> {
        let private_key_hex = generate_private_key(biguint);
        let secret_key = SecretKey::from_slice(&hex::decode(private_key_hex)?)?;

        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Ok(Wallet::new(&secret_key, &public_key))
    }
    
    pub fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    }
}


pub mod eth_wallet_simple_u64 {
    
    use rand::Rng;
    use rand::RngCore;
    use once_cell::sync::Lazy;

    use tiny_keccak::keccak256;
    use web3::types::Address;
    use std::sync::Mutex;

    use secp256k1::{PublicKey, Secp256k1, SecretKey};
    static SECRET_KEY_BYTES: Lazy<Mutex<[u8; 32]>> = Lazy::new(|| {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes[0..24]);
        Mutex::new(bytes)
    });


    #[derive(Debug)]
    pub struct Wallet {
        pub secret_key: String,
        pub public_key: String,
        pub address: String,
    }

    impl Wallet {
        pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
            let addr: Address = public_key_address(&public_key);
            Wallet {
                secret_key: hex::encode(&secret_key.secret_bytes()),
                public_key: public_key.to_string(),
                address: hex::encode(addr),
            }
        }
    }

    pub fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    }

    pub struct Generator {
        pub current_u64: u64,
    }
    
    impl Generator {
        pub fn new() -> Generator {
            let mut gen = Generator {
                current_u64: 1,  // initialized to 1
            };
            gen.randomize();
            gen
        }
        
        pub fn increment(&mut self) {
            self.current_u64 += 1;
        }
        
        pub fn randomize(&mut self) {
            let mut rng = rand::thread_rng();
            self.current_u64 = rng.gen::<u64>();
        }
    }
    
    pub fn generate_ecdsa_key_pair_from_private_key(u64_value: u64) -> Result<Wallet, Box<dyn std::error::Error>> {
        {
            let mut secret_key_bytes = SECRET_KEY_BYTES.lock().unwrap();
            secret_key_bytes[24..32].copy_from_slice(&u64_value.to_be_bytes());
        }
        let secret_key = SecretKey::from_slice(&*SECRET_KEY_BYTES.lock().unwrap())?;
        
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Ok(Wallet::new(&secret_key, &public_key))
    }
        
}


pub mod eth_wallet_u64_contained {
    
    use rand::Rng;
    use rand::RngCore;
    use once_cell::sync::Lazy;

    use tiny_keccak::keccak256;
    use web3::types::Address;
    use std::sync::Mutex;

    use secp256k1::{PublicKey, Secp256k1, SecretKey};
    static SECRET_KEY_BYTES: Lazy<Mutex<[u8; 32]>> = Lazy::new(|| {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes[0..24]);
        Mutex::new(bytes)
    });    
    static CURRENT_UINT64: Lazy<Mutex<u64>> = Lazy::new(|| {
        let mut rng = rand::thread_rng();
        Mutex::new(rng.gen::<u64>())
    });


    #[derive(Debug)]
    pub struct Wallet {
        pub secret_key: String,
        pub public_key: String,
        pub address: String,
    }

    impl Wallet {
        pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
            let addr: Address = public_key_address(&public_key);
            Wallet {
                secret_key: hex::encode(&secret_key.secret_bytes()),
                public_key: public_key.to_string(),
                address: hex::encode(addr),
            }
        }
        
    }

    pub fn set_secret_key_bytes() {
        let mut secret_key_bytes = SECRET_KEY_BYTES.lock().unwrap();
        rand::thread_rng().fill_bytes(&mut secret_key_bytes[0..24]);
    }
    
    pub fn set_current_u64() {
        let mut current_u64 = CURRENT_UINT64.lock().unwrap();
        *current_u64 = rand::thread_rng().gen::<u64>();
    }
    
    pub fn increment_current_u64() -> u64{
        let mut current_u64 = CURRENT_UINT64.lock().unwrap();
        *current_u64 += 1;
        *current_u64
    }
    
    pub fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        let hash = keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    }

    pub fn generate_ecdsa_key_pair_from_private_key() -> Result<Wallet, Box<dyn std::error::Error>> {
        let u64_value: u64 = increment_current_u64();
        let mut secret_key_bytes = SECRET_KEY_BYTES.lock().unwrap();
        secret_key_bytes[24..32].copy_from_slice(&u64_value.to_be_bytes());
        let secret_key = SecretKey::from_slice(&*secret_key_bytes)?;
        
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Ok(Wallet::new(&secret_key, &public_key))
    }
        
}


#[cfg(test)]
mod tests {
    use super::eth_wallet;
    use super::eth_wallet_simple;
    use super::eth_wallet_simple_u64;
    use super::eth_wallet_u64_contained;
    use super::polka_wallet;
    use std::time::Instant;
    use eth_checksum;

    fn g_wallet(n_wallets: i32) {
        for _ in 0..n_wallets {
            let _wallet = eth_wallet::generate_random_wallet();
        }
    }

    fn s_wallet(n_wallets: i32) { 
        let mut biguint_generator = eth_wallet_simple::Generator::new();
        for _ in 0..n_wallets {
            let _wallet = eth_wallet_simple::generate_ecdsa_key_pair_from_private_key(biguint_generator.current_biguint.clone()).unwrap();
            biguint_generator.increment();
        }
    }

    fn s_64_wallet(n_wallets: i32) { 
        let mut u64_generator = eth_wallet_simple_u64::Generator::new();
        for _ in 0..n_wallets {
            let _wallet = eth_wallet_simple_u64::generate_ecdsa_key_pair_from_private_key(u64_generator.current_u64).unwrap();
            u64_generator.increment();
        }
    }

    fn s_64_reduced(n_wallets: i32) {
        for _ in 0..n_wallets {
            let _wallet = eth_wallet_u64_contained::generate_ecdsa_key_pair_from_private_key().unwrap();
        }
    }

    fn polka_wallet(n_wallets: i32) {
        for _ in 0..n_wallets {
            let _wallet = polka_wallet::generate_random_wallet();
        }
    }

    #[test]
    fn wallet_generation_speed() {
        let n_wallets = 1000;

        let to_test = [
                ("g_wallet", g_wallet as fn(i32)), 
                ("s_wallet", s_wallet as fn(i32)), 
                ("s_64_wallet", s_64_wallet as fn(i32)),
                ("s_64_reduced", s_64_reduced as fn(i32)),
                ("polka_wallet", polka_wallet as fn(i32))
            ];

        println!("Generating {} wallets for every function", n_wallets);
        for (name, func) in to_test.iter() {
            let start = Instant::now();

            func(n_wallets);

            let duration = start.elapsed();
            let duration_in_secs = duration.as_secs_f64();
            let wallets_per_second = n_wallets as f64 / duration_in_secs;

            println!("{} in {:.2} seconds, {:.2} wallets/second", name, duration_in_secs, wallets_per_second);
        }

    }

}