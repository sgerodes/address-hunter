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

    let mut wallet_creation_time: i64 = 0;
    let mut vanity_check_time: i64 = 0;
    let mut before_wallet: Instant = Instant::now();
    let mut after_wallet: Instant = Instant::now();
    let mut after_vanity: Instant = Instant::now();
    loop {
        while loop_counter < efficiency_count {

            before_wallet = Instant::now();
            let wallet = address::eth_wallet::generate_random_wallet();
            after_wallet = Instant::now();
            
            let vanity_result: VanityResult = vanity_generator::does_address_meet_criteria(&wallet);
            after_vanity = Instant::now();
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
        wallet_creation_time += after_wallet.duration_since(before_wallet).as_nanos() as i64;
        vanity_check_time += after_vanity.duration_since(after_wallet).as_nanos() as i64;
        println!("wallet_creation_time / vanity_check_time:  {} ", wallet_creation_time / vanity_check_time);
        wallet_creation_time = 0;
        vanity_check_time = 0;

        total_adresses_searched += efficiency_count;
        println!("Total searched {} addresses. Loops per second: {}", total_adresses_searched, efficiency_count as f64 / start.elapsed().as_secs_f64());
        loop_counter = 0;
        start = Instant::now();
    }
}
