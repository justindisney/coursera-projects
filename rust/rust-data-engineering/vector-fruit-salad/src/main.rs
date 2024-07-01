/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::io;

fn main() {
    let predefined_fruits = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    let mut fruits = Vec::new();
    let mut input = String::new();
    let mut rng = thread_rng();

    println!("Enter fruit names or a number to auto-populate the list of fruits (type 'q' to quit):");

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let trimmed_input = input.trim();

        if trimmed_input.eq_ignore_ascii_case("q") {
            break;
        }

        if let Ok(number) = trimmed_input.parse::<usize>() {
            for _ in 0..number {
                if let Some(&random_fruit) = predefined_fruits.choose(&mut rng) {
                    fruits.push(random_fruit.to_string());
                }
            }
            break;
        } else {
            fruits.push(trimmed_input.to_string());
        }
    }

    // Scramble (shuffle) the fruit
    fruits.shuffle(&mut rng);

    if let Some(choice) = fruits.choose(&mut rng) {
        println!("Randomly chosen fruit: {}", choice);
    } else {
        println!("The list is empty!");
    }

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
