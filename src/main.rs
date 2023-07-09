#[warn(unused_imports)]
#[warn(dead_code)]
use std::time::Instant;
use crate::vanity_generator::VanityResult;

mod address;
mod vanity_generator;
mod database;


fn main() {

    println!("Vanity Generaor started!");
    let mut loop_counter = 0;
    let mut total_adresses_searched = 0;
    let mut start = Instant::now();
    let efficiency_count = 1_000_000;
    loop {
        while loop_counter < efficiency_count {

            let wallet = address::eth_wallet::generate_random_wallet();
            
            let vanity_result: VanityResult = vanity_generator::does_address_meet_criteria(&wallet);
            if vanity_result.met_criteria {
                println!("{}: {:?}", vanity_result.wallet.address, vanity_result.matched_rule);
                let insertion_result = database::database::write_eth_wallet(&vanity_result);
                match insertion_result {
                    Ok(_) => {
                        println!("Wrote to DB {}", vanity_result.wallet.address);
                    },
                    Err(e) => {
                        println!("Error writing to DB: {}", e);
                    }
                }
            }
            loop_counter += 1;
        }
        total_adresses_searched += efficiency_count;
        println!("Total searched {} addresses. Loops per second: {}", total_adresses_searched, efficiency_count as f64 / start.elapsed().as_secs_f64());
        loop_counter = 0;
        start = Instant::now();
    }
}
