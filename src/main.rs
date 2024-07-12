mod word_list;

use chrono::prelude::*;
use colored::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io;

fn main() {
    loop {
        let word_pairs = word_list::read_word_pairs("word_pairs.json");

        println!("Choose practice mode:(1) English words (2) Numeric digits");
        let mut mode_input = String::new();
        io::stdin()
            .read_line(&mut mode_input)
            .expect("Failed to read line");
        let mode = mode_input.trim();

        match mode {
            "1" => practice_words(&word_pairs),
            "2" => practice_numbers(),
            _ => println!("Invalid mode"),
        }
    }
}

fn generate_random_word_pair(word_pairs: &[word_list::WordPair]) -> &word_list::WordPair {
    let mut rng = rand::thread_rng();
    word_pairs.choose(&mut rng).expect("Word list is empty")
}

fn generate_random_number(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect()
}

fn practice_words(word_pairs: &[word_list::WordPair]) {
    for _ in 0..5 {
        let target = generate_random_word_pair(word_pairs);
        println!("Please type the following word: {}", target.english);

        let start = Utc::now();
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let end = Utc::now();

        let duration = end - start;
        let trimmed_input = user_input.trim();

        if trimmed_input == target.english {
            println!(
                "{}",
                format!("Correct! Time taken: {} seconds", duration.num_seconds()).green()
            );
        } else {
            println!(
                "{}",
                format!("Incorrect! You typed: {}", trimmed_input).red()
            );
        }

        println!("The Chinese meaning is: {}", target.chinese);

        io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");
    }
}

fn practice_numbers() {
    for _ in 0..5 {
        let target = generate_random_number(10);
        println!("Please type the following number: {}", target);

        let start = Utc::now();
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let end = Utc::now();

        let duration = end - start;
        let trimmed_input = user_input.trim();

        if trimmed_input == target {
            println!(
                "{}",
                format!("Correct! Time taken: {} seconds", duration.num_seconds()).green()
            );
        } else {
            println!(
                "{}",
                format!("Incorrect! You typed: {}", trimmed_input).red()
            );
        }

        io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");
    }
}
