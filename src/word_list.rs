use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WordPair {
    pub english: String,
    pub chinese: String,
}

pub fn read_word_pairs(filename: &str) -> Vec<WordPair> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to read file")
}
