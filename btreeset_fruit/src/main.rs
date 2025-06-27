use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeSet;
use std::io;


fn remove_fruit(fruit_set: &mut BTreeSet<&str>) {
    println!("What fruit would you like to remove: )");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let fruit = input.trim();
    if fruit_set.remove(fruit) {
        println!("Remove {} from the set.", fruit);
    } else {
        println!("{} is not in the set.", fruit);
    }
}

fn main() {
    let fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
    ];
    let amounts = [1, 3, 5, 7, 9];

    let mut rng = thread_rng();

    for amount in amounts.iter() {
        let mut fruit_set = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
            fruit_set.insert(fruit);
            if fruit_set.len() >= *amount {
                break;
            }
        }

        println!("{}: {:?}", amount, fruit_set);
        remove_fruit(&mut fruit_set);
        println!("Set after removal: {:?}", fruit_set);
    
    println!("All unquie fruits in reverse order:");
    for fruit in fruit_set.iter().rev() {
        println!("{}", fruit);
    }
        
    }
}
