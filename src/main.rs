use std::fs::File;
use std::io::{self, BufRead};

use indicatif::ProgressBar;

// const WORD_FILENAME: &str = "words_alpha.txt";
const WORD_FILENAME: &str = "words_5.txt"; // used for now for faster debug
const OUT_FILENAME: &str = "cliques.txt";

fn main() {
    // load file with words and take every with five letters
    let worlds_five: Vec<String> =
        io::BufReader::new(File::open(WORD_FILENAME).expect("Could not find input file."))
            .lines()
            .filter_map(|x| x.ok())
            .filter(|x| x.len() == 5)
            .collect();

    print!("{:?}", worlds_five);
}
