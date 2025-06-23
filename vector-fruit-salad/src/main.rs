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

fn input_fruit() -> String{
    println!("Please enter a fruit name:");
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input.trim().to_string() // trim() removes whitespace from the beginning and end of the string
}


fn main() {
    let mut user_input = input_fruit();
    let mut fruit: Vec<String> = vec![
        "Orange".to_string(),
        "Fig".to_string(),
        "Pomegranate".to_string(),
        "Cherry".to_string(),
        "Apple".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
    ];
    fruit.push(user_input);
    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}