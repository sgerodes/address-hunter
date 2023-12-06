pub mod database {

    use crate::vanity_generator::VanityResult;
    use dotenv::dotenv;
    use std::env;
    use postgres::{Client, NoTls, Error};

    pub fn create_client() -> Result<Client, Error> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Ok(Client::connect(&database_url, NoTls)?)
    }

    pub trait DatabaseHandler {
        fn write_eth_wallet(&self, vanity_result: &VanityResult) -> Result<(), Error>;
    }

    // Actual database
   pub struct RealDatabase;
    impl DatabaseHandler for RealDatabase {
        fn write_eth_wallet(&self, vanity_result: &VanityResult) -> Result<(), Error> {
            let mut client: Client = create_client()?;
            client.execute(
                "INSERT INTO eth (public_address, public_address_checksummed, private_key, info, entropy_coefficient, proximity_coefficient,  entropy_coefficient_checksummed, proximity_coefficient_checksummed) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[&vanity_result.wallet.address, &vanity_result.wallet.address_checksummed, &vanity_result.wallet.secret_key, &vanity_result.matched_rule, &vanity_result.entropy_coefficient, &vanity_result.proximity_coefficient, &vanity_result.entropy_coefficient_checksummed, &vanity_result.proximity_coefficient_checksummed],
            )?;
            Ok(())
        }
    }

    // In-memory emulation
    pub struct MockDatabase;
    impl DatabaseHandler for MockDatabase {
        fn write_eth_wallet(&self, vanity_result: &VanityResult) -> Result<(), Error> {
            // In-memory writing logic here
            println!("Mock Database: Writing to emulated database {}", vanity_result);
            Ok(())
        }

    }
}

pub mod polkadot_database {

    use crate::vanity_generator::PolkadotVanityResult;
    use dotenv::dotenv;
    use std::env;
    use postgres::{Client, NoTls, Error};

    pub fn create_client() -> Result<Client, Error> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Ok(Client::connect(&database_url, NoTls)?)
    }

    pub trait DatabaseHandler {
        fn write_eth_wallet(&self, vanity_result: &PolkadotVanityResult) -> Result<(), Error>;
    }

    // Actual database
   pub struct RealDatabase;
    impl DatabaseHandler for RealDatabase {
        fn write_eth_wallet(&self, vanity_result: &PolkadotVanityResult) -> Result<(), Error> {
            let mut client: Client = create_client()?;
            client.execute(
                "INSERT INTO polkadot (public_address, private_key, info, entropy_coefficient, proximity_coefficient) VALUES ($1, $2, $3, $4, $5)",
                &[&vanity_result.wallet.address, &vanity_result.wallet.secret_key, &vanity_result.matched_rule, &vanity_result.entropy_coefficient, &vanity_result.proximity_coefficient],
            )?;
            Ok(())
        }
    }

    // In-memory emulation
    pub struct MockDatabase;
    impl DatabaseHandler for MockDatabase {
        fn write_eth_wallet(&self, vanity_result: &PolkadotVanityResult) -> Result<(), Error> {
            // In-memory writing logic here
            println!("Mock Database: Writing to emulated database {}", vanity_result);
            Ok(())
        }

    }
}

#[cfg(test)]
mod tests {

    use super::database;

    #[test]
    fn test_write_eth_wallet() {
        let result = database::create_client();
        assert!(result.is_ok());
    }
}