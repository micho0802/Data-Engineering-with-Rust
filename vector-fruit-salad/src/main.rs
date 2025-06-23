/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

// SliceRandom trait is a part of rand crate, which allows you to perform random operations on slices(&[T] and &mut [T]), 
// such as shuffling.
use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::io;
use rand::prelude::IndexedRandom; // This trait provides the choose method to select a random element from a slice.


fn input_fruit() -> String {
    println!("Please enter a fruit name:");
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input.trim().to_string() // trim() removes whitespace from the beginning and end of the string
}

fn number_of_fruits() -> i8 {
    loop {
        println!("How many fruits would you like to add to your fruit salad? (1-8)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse::<i8>() {
            Ok(num) if num >= 1 && num <= 5 => return num,
            _ => println!("Please enter a valid number between 1 and 5."),
        }
    }
}

fn main() {
    let mut user_input = input_fruit();
    let mut available_fruit:Vec<String>  = vec![
        "Orange".to_string(),
        "Fig".to_string(),
        "Pomegranate".to_string(),
        "Cherry".to_string(),
        "Apple".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
    ];
    available_fruit.push(user_input);
    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let num_to_add = number_of_fruits();
    let mut selected_fruit: Vec<String> = Vec::new();

    for _ in 0..num_to_add {
        if let Some(fruit) = available_fruit.choose(&mut rng) {
            selected_fruit.push(fruit.to_string());
        }
    }

    let mut fruit_salad: Vec<String> = vec!["Apple".to_string(), "Cherry".to_string(), "Orange".to_string()];
    fruit_salad.extend(selected_fruit);
    fruit_salad.shuffle(&mut rng);


    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit_salad.iter().enumerate() {
        if i != fruit_salad.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Choose a random fruit
    if let Some(random_fruit) = available_fruit.choose(&mut rng) {
        println!("Your random fruit is: {}", random_fruit);
    } else {
        println!("No fruit available to choose from.");
    }
}