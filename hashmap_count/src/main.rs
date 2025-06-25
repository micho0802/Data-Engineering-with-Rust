/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;
use std::io;

fn ask_user_input_number() -> Vec<i32> {
    let mut input = String::new();
    println!("Enter an array of numbers separated by spaces:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn ask_user_input_words() -> Vec<String> {
    let mut input = String::new();
    println!("Enter an array of words separated by spaces:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn logic_number(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0); // return a mutable reference i.e. &mut u32
        *frequency += 1;
    }

    let mut result: Vec<(i32, u32)> = frequencies.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1)); // sort by frequency in descending order
    result

}

fn logic_words(words: Vec<String>) -> Vec<(String, u32)> {
    let mut frequencies = HashMap::new();

    for word in words {
        let frequency = frequencies.entry(word).or_insert(0);
        *frequency += 1;
    }

    let mut result: Vec<(String, u32)> = frequencies.into_iter().collect();

    result.sort_by(|a, b| b.1.cmp(&a.1));
    result
}

fn main() {
    let numbers = ask_user_input_number();
    let result_numbers = logic_number(numbers);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result_numbers
    );

    let words = ask_user_input_words();
    let result_words = logic_words(words);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each word in the vector is: {:?}",
        result_words
    );
}
