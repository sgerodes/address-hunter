#[warn(unused_imports)]
#[warn(dead_code)]
use std::time::Instant;
use std::env;
use crate::vanity_generator::VanityResult;
use crate::address_utils::address_utils::calculate_proximity_coefficient;
use crate::address_utils::address_utils::calculate_entropy;
use rayon::prelude::*;
use dotenv::dotenv;

mod address;
mod vanity_generator;
mod database;
mod address_utils;
mod percentile_heap;


fn main() {
    dotenv().ok();


    let process_count = env::var("PROCESS_COUNT")
    .unwrap_or_else(|_| "1".to_string())
    .parse()
    .unwrap_or_else(|_| {
        println!("Failed to parse PROCESS_COUNT, defaulting to 1");
        1
    });
    println!("Starting {} processes", process_count);

    (0..process_count).into_par_iter().for_each(|task_id| {
        run_vanity(task_id);
    });
}


fn run_vanity(task_id: i32) {

    println!("Process {}: Vanity Generaor started!", task_id);

    let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "localdev".to_string());
    let db: Box<dyn database::database::DatabaseHandler> = if environment == "prod" {
        println!("Using prod database");
        Box::new(database::database::RealDatabase)
    } else {
        println!("Using mock database");
        Box::new(database::database::MockDatabase)
    };

    let mut loop_counter = 0;
    let mut total_adresses_searched = 0;
    let mut start = Instant::now();
    let efficiency_count = env::var("EFFICIENCY_COUNT")
        .unwrap_or_else(|_| "1000000".to_string())
        .parse()
        .unwrap_or_else(|_| {
            println!("Failed to parse EFFICIENCY_COUNT, defaulting to 1000000");
            1000000
        }); 
    println!("Process {}: Efficiency count set to: {}", task_id, efficiency_count);

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
                println!("Process {}: {} - {:?} - Entropy {:.2}, Proximity {:.2}, CS Entropy {:.2}, CS Proximity {:.2}", task_id, vanity_result.wallet.address_checksummed, vanity_result.matched_rule, &vanity_result.entropy_coefficient, &vanity_result.proximity_coefficient, &vanity_result.entropy_coefficient_checksummed, &vanity_result.proximity_coefficient_checksummed);
                match db.write_eth_wallet(&vanity_result) {
                    Ok(_) => {
                        println!("Process {}: Wrote to DB {}", task_id, vanity_result.wallet.address_checksummed);
                    },
                    Err(e) => {
                        println!("Process {}: Error writing to DB: {}", task_id, e);
                    }
                }
            }
            loop_counter += 1;
        }
        wallet_creation_time += after_wallet.duration_since(before_wallet).as_nanos() as i64;
        vanity_check_time += after_vanity.duration_since(after_wallet).as_nanos() as i64;
        println!("Process {}: wallet_creation_time / vanity_check_time: {} ", task_id, wallet_creation_time as f64 / vanity_check_time as f64);
        wallet_creation_time = 0;
        vanity_check_time = 0;

        total_adresses_searched += efficiency_count;
        println!("Process {}: Total searched {} addresses. Loops per second: {}", task_id, total_adresses_searched, efficiency_count as f64 / start.elapsed().as_secs_f64());
        loop_counter = 0;
        start = Instant::now();
    }
}
