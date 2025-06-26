use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::io;

fn ask_user_number() -> u8 {
    println!("How many number of random fruit?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, defaulting to 9.");
            9
        }
    }
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
    let number_of_fruits = ask_user_number();
    println!("Generating {} random fruits...", number_of_fruits);
    for _ in 0..number_of_fruits {
        fruit_set.insert(generate_fruit());
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());

    let mut fruit_counts: HashMap<&str, u32> = HashMap::new();

    for _ in 0..number_of_fruits {
        let fruit = generate_fruit();
        *fruit_counts.entry(fruit).or_insert(0) += 1;
    }

    println!("Fruit counts:");
    for (fruit, count) in &fruit_counts {
        println!("{}: {}", fruit, count);
    }
    println!(
        "\nNumber of unique fruits generated: {}",
        fruit_counts.len()
    );
}
