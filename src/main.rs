use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

// const WORD_FILENAME: &str = "words_alpha.txt";
const WORD_FILENAME: &str = "words_5.txt"; // used for now for faster debug
const OUT_FILENAME: &str = "cliques.txt";
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    // load file with words and take every with five letters
    println!("Load file...");
    let mut worlds_five: Vec<String> =
        io::BufReader::new(File::open(WORD_FILENAME).expect("Could not find input file."))
            .lines()
            .filter_map(|x| x.ok())
            .filter(|x| x.len() == 5)
            .map(|x| x.to_lowercase())
            .collect();
    println!("Amount of words:\t{:?}", worlds_five.len());

    println!("Filter words with repeating characters...");
    worlds_five.retain(|x| !word_repeat_char(x, 5));
    println!("Amount of words:\t{:?}", worlds_five.len());

    println!("Map words to u32 representative...");

    // println!("{:?}", worlds_five);
}

fn word_repeat_char(w: &str, word_size: usize) -> bool {
    let mut map: HashMap<char, bool> = HashMap::with_capacity(word_size);
    for c in w.chars() {
        if map.get(&c).is_some() {
            return true;
        }
        map.insert(c, true);
    }
    false
}
