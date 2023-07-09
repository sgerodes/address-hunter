#[warn(unused_imports)]
#[warn(dead_code)]
use std::time::Instant;

mod address;
mod vanity_generator;


fn main() {

    println!("Vanity Generaor started!");
    let mut loop_counter = 0;
    let mut total_adresses_searched = 0;
    let mut start = Instant::now();
    let efficiency_count = 1_000_000;
    loop {
        while loop_counter < efficiency_count {

            let wallet = address::eth_wallet::generate_random_wallet();
            
            vanity_generator::does_address_meet_criteria(&wallet.address);
            loop_counter += 1;
        }
        total_adresses_searched += efficiency_count;
        println!("Total searched {} addresses. Loops per second: {}", total_adresses_searched, efficiency_count as f64 / start.elapsed().as_secs_f64());
        loop_counter = 0;
        start = Instant::now();
    }
}
