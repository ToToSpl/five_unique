use indicatif::ProgressBar;
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
    let mut words_five: Vec<String> =
        io::BufReader::new(File::open(WORD_FILENAME).expect("Could not find input file."))
            .lines()
            .filter_map(|x| x.ok())
            .filter(|x| x.len() == 5)
            .map(|x| x.to_lowercase())
            .collect();
    words_five.sort_unstable();
    println!("Amount of words:\t{:?}", words_five.len());

    println!("Filter words with repeating characters...");
    words_five.retain(|x| !word_repeat_char(x, 5));
    println!("Amount of words:\t{:?}", words_five.len());

    println!("Map words to u32 representative...");
    let coded_words: Vec<u32> = words_five.iter().map(|x| word_to_u32(x)).collect();

    println!("Create graph...");
    let graph = create_graph(&coded_words);

    println!("Finding cliques...");
    let size = graph.len();
    let mut cliques: Vec<Vec<String>> = Vec::new();
    let mut clique: Vec<&String> = Vec::with_capacity(5);
    let mut words_combined: Vec<u32> = vec![0; 4];
    let pb = ProgressBar::new(coded_words.len().try_into().unwrap());
    for i in 0..size {
        if graph[i].len() < 4 {
            continue;
        }
        clique.push(&words_five[i]); // 1
        words_combined[0] = coded_words[i];
        for j in &graph[i] {
            if words_combined[0] & coded_words[*j] != 0 {
                continue;
            }
            if graph[*j].len() < 3 {
                continue;
            }
            words_combined[1] = words_combined[0] | coded_words[*j];
            clique.push(&words_five[*j]); // 2

            for k in &graph[*j] {
                if words_combined[1] & coded_words[*k] != 0 {
                    continue;
                }
                if graph[*k].len() < 2 {
                    continue;
                }
                words_combined[2] = words_combined[1] | coded_words[*k];
                clique.push(&words_five[*k]); // 3
                for l in &graph[*k] {
                    if words_combined[2] & coded_words[*l] != 0 {
                        continue;
                    }
                    if graph[*l].len() < 1 {
                        continue;
                    }
                    words_combined[3] = words_combined[2] | coded_words[*l];
                    clique.push(&words_five[*l]); // 4

                    for m in &graph[*l] {
                        if words_combined[3] & coded_words[*m] != 0 {
                            continue;
                        }
                        clique.push(&words_five[*m]);
                        cliques.push(clique.clone().into_iter().cloned().collect());
                        println!("Found:\t{:?}", cliques.last());
                        clique.pop();
                    }
                    clique.pop();
                }
                clique.pop();
            }
            clique.pop();
        }
        clique.pop();
        pb.inc(1);
    }

    // print_coded(&words_five, &coded_words);
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

fn word_to_u32(w: &str) -> u32 {
    let mut coded: u32 = 0;
    for c in ALPHABET.chars() {
        coded = (coded << 1) + if w.contains(c) { 1 } else { 0 };
    }
    coded
}

fn print_coded(worlds: &Vec<String>, coded: &Vec<u32>) {
    println!("world\tA B C D E F G H I J K L M N O P Q R S T U V W X Y Z");
    for i in 0..worlds.len() {
        print!("{:}\t", worlds[i]);
        let mut check: u32 = 1 << ALPHABET.len() - 1;
        for j in 0..ALPHABET.len() {
            if coded[i] & check != 0 {
                print!("{:} ", ALPHABET.chars().nth(j).unwrap());
            } else {
                print!("_ ");
            }
            check = check >> 1;
        }
        println!("");
    }
}

fn create_graph(coded: &Vec<u32>) -> Vec<Vec<usize>> {
    let mut graph = vec![Vec::new(); coded.len()];
    for i in 0..coded.len() {
        for j in i + 1..coded.len() {
            if coded[i] & coded[j] == 0 {
                graph[i].push(j);
            }
        }
    }
    graph
}
