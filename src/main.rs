#[warn(unused_imports)]
#[warn(dead_code)]

mod address_manager;
mod hunter;

fn main() {
    address_manager::create_key_bundle();
    println!("Hello, world!");
}

#[cfg(test)]
mod benches;