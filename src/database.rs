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

    pub fn write_eth_wallet(vanity_result: &VanityResult) -> Result<(), Error> {
        let mut client: Client = create_client()?;
        client.execute(
            "INSERT INTO eth (public_address, private_key, info) VALUES ($1, $2, $3)",
            &[&vanity_result.wallet.address, &vanity_result.wallet.secret_key, &vanity_result.matched_rule],
        )?;
        Ok(())
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