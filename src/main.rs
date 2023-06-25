#[warn(unused_imports)]
#[warn(dead_code)]
use std::time::Instant;

mod address_manager;
mod hunter;
mod address;

fn main() {
    println!("Hunter started!");
    let mut loop_counter = 0;
    let mut total_adresses_searched = 0;
    let mut start = Instant::now();
    let efficiency_count = 100;
    loop {
        while loop_counter < efficiency_count {
            let key_bundle = address_manager::create_key_bundle();
            hunter::does_address_meet_criteria(&key_bundle.public_address_no_0x);
            loop_counter += 1;
        }
        total_adresses_searched += efficiency_count;
        println!("Total searched {} addresses. Loops per second: {}", total_adresses_searched, efficiency_count as f64 / start.elapsed().as_secs_f64());
        loop_counter = 0;
        start = Instant::now();
    }
}
