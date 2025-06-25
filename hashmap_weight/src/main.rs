use std::collections::HashMap;
use std::io::{self, Write};

fn ask_user_input() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    println!("Enter programming languages and their creation years (type 'done' to finish):");

    loop {
        print!("Language name: ");
        io::stdout().flush().unwrap();
        let mut language = String::new();
        io::stdin().read_line(&mut language).unwrap();
        let language = language.trim();
        if language.eq_ignore_ascii_case("done") {
            break;
        }
        print!("Creation year: ");
        io::stdout().flush().unwrap();
        let mut year = String::new();
        io::stdin().read_line(&mut year).unwrap();
        let year = match year.trim().parse::<i32>() {
            Ok(y) => y,
            Err(_) => {
                println!("Invalid year. Please enter a valid integer.");
                continue;
            }
        };
        languages.insert(language.to_string(), year);
    }
    languages

}

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);
    
    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    // Subtract the creation year from 2024 to get the number of years active.
    for year in years_active.values_mut() {
        *year = 2025 - *year;
    }
    
    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);
    
    let mut weights = HashMap::new();
    
    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1;  // weight between 1 and 100
        weights.insert(language.to_string(), weight);
    } 
    weights
}

fn main() {
    let mut init_languages = init_languages();
    
    let user_languages = ask_user_input();
    for (language, year) in user_languages {
        init_languages.insert(language, year);
    }

    let weights = calculate_weights(&mut init_languages);
    
    let mut sorted: Vec<_> = weights.iter().collect();
    sorted.sort_by(|a, b| a.1.cmp(b.1));

    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in sorted {
        println!("{}: {}", language, weight);
    }

    
}
