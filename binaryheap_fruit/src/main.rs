use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::BinaryHeap;
use std::io;

#[derive(Eq, PartialEq)]
enum Fruit {
    Fig,
    Other(String),
}

// We define Figs as the highest priority by implementing Ord
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn remove_fruit(heap: BinaryHeap<Fruit>, fruit_to_remove: &str) -> BinaryHeap<Fruit> {
    let mut removed = false;
    let mut new_heap = BinaryHeap::new();

    for fruit in heap.into_sorted_vec() {
        let is_match = match (&fruit, fruit_to_remove.to_lowercase().as_str()) {
            (Fruit::Fig, "fig") => true,
            (Fruit::Other(name), s) if name.to_lowercase() == s => true,
            _ => false,
        };
        if is_match && !removed {
            removed = true;
            continue; // Remove only the first match
        }
        new_heap.push(fruit);
    }
    new_heap
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = thread_rng();
    let fruits = vec![
        "Apple", "Orange", "Pear", "Peach", "Banana", "Fig", "Fig", "Fig", "Fig",
    ];
    let mut fruit_salad = BinaryHeap::new();

    let mut figs_count = 0;
    while figs_count < 2 {
        let fruit = fruits.choose(&mut rng).unwrap();
        if *fruit == "Fig" {
            figs_count += 1;
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    fruit_salad
}

fn main() {
    let fruit_salad = generate_fruit_salad();
    println!("Random Fruit Salad With Two Servings of Figs:");
    for fruit in fruit_salad.into_sorted_vec() {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(fruit_name) => println!("{}", fruit_name),
        }
    }
}
