use indicatif::ProgressBar;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::time::Instant;

// const WORD_FILENAME: &str = "words_alpha.txt";
const WORD_FILENAME: &str = "words_5.txt"; // use for faster debug
const OUT_FILE: &str = "cliques.txt";
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let start = Instant::now();
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
    println!("Amount of five letter words:\t{:?}", words_five.len());

    println!("Filter words with repeating characters...");
    words_five.retain(|x| !word_repeat_char(x, 5));
    println!("Amount of words:\t{:?}", words_five.len());

    println!("Map words to u32 representative...");
    let coded_words: Vec<u32> = words_five.iter().map(|x| word_to_u32(x)).collect();

    println!("Compress anagrams...");
    let (coded_words, map_anagrams) = compress_anagrams(&words_five, &coded_words);
    println!("Amount of words:\t{:?}", coded_words.len());

    println!("Create graph...");
    let graph = create_graph(&coded_words);

    println!("Finding cliques...");
    let mut cliques: Vec<Vec<String>> = Vec::new();
    let mut words_combined: Vec<u32> = vec![0; 4];
    let pb = ProgressBar::new(coded_words.len().try_into().unwrap());
    for i in 0..graph.len() {
        if graph[i].len() < 4 {
            pb.inc(1);
            continue;
        }
        words_combined[0] = coded_words[i];
        for j in &graph[i] {
            if words_combined[0] & coded_words[*j] != 0 {
                continue;
            }
            words_combined[1] = words_combined[0] | coded_words[*j];

            for k in &graph[*j] {
                if words_combined[1] & coded_words[*k] != 0 {
                    continue;
                }
                words_combined[2] = words_combined[1] | coded_words[*k];
                for l in &graph[*k] {
                    if words_combined[2] & coded_words[*l] != 0 {
                        continue;
                    }
                    words_combined[3] = words_combined[2] | coded_words[*l];
                    for m in &graph[*l] {
                        if words_combined[3] & coded_words[*m] != 0 {
                            continue;
                        }
                        cliques.append(&mut cliques_from_anagram(
                            &vec![
                                coded_words[i],
                                coded_words[*j],
                                coded_words[*k],
                                coded_words[*l],
                                coded_words[*m],
                            ],
                            &map_anagrams,
                        ));
                    }
                }
            }
        }
        pb.inc(1);
    }

    println!("Found {:} cliques. Saving...", cliques.len());
    {
        let mut file = File::create(OUT_FILE).unwrap();
        for clique in cliques.into_iter() {
            file.write((clique.join("\t") + "\n").as_bytes()).unwrap();
        }
    }
    println!("Time taken: {:?}", start.elapsed());
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

fn compress_anagrams(
    words: &Vec<String>,
    coded: &Vec<u32>,
) -> (Vec<u32>, HashMap<u32, Vec<String>>) {
    let mut map = HashMap::new();
    for (i, code) in coded.iter().enumerate() {
        match map.entry(*code) {
            Vacant(e) => {
                e.insert(vec![words[i].clone()]);
            }
            Occupied(mut e) => {
                e.get_mut().push(words[i].clone());
            }
        }
    }
    let mut keys: Vec<u32> = map.keys().cloned().collect();
    keys.sort_unstable();
    (keys, map)
}

fn cliques_from_anagram(anagram: &Vec<u32>, map: &HashMap<u32, Vec<String>>) -> Vec<Vec<String>> {
    let mut cliques: Vec<Vec<String>> = Vec::new();
    let mut clique = Vec::new();
    for ni in map.get(&anagram[0]).unwrap() {
        clique.push(ni);
        for nj in map.get(&anagram[1]).unwrap() {
            clique.push(nj);
            for nk in map.get(&anagram[2]).unwrap() {
                clique.push(nk);
                for nl in map.get(&anagram[3]).unwrap() {
                    clique.push(nl);
                    for nm in map.get(&anagram[4]).unwrap() {
                        clique.push(nm);
                        let mut c = clique.clone();
                        c.sort_unstable();
                        cliques.push(c.into_iter().cloned().collect());
                        clique.pop();
                    }
                    clique.pop();
                }
                clique.pop();
            }
            clique.pop();
        }
        clique.pop();
    }
    cliques
}

#[allow(dead_code)]
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
