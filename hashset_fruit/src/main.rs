use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::io;

fn ask_user_number() -> u8 {
    println!("How many number of random fruit?");
    
}

fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    println!("Generating 100 random fruits...");
    for _ in 0..100 {
        fruit_set.insert(generate_fruit());
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
}