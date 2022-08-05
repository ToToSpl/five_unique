use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

use indicatif::ProgressBar;

struct NodeGraph<'a> {
    node: &'a String,
    paths: Vec<&'a String>,
}

fn main() {
    let file = File::open("words_alpha.txt").expect("Failed to open words_alpha file");
    let words_five_letter = get_all_five_words(file);
    println!("all five letter words: {}", words_five_letter.len());

    let words_five_letter = remove_repeat_letter_words(words_five_letter);
    println!("without repeating letters: {}", words_five_letter.len());

    let sorted_map = sort_anagrams(&words_five_letter);
    println!("with reduced anagrams: {}", sorted_map.len());
    // println!("{:?}", sorted_map.into_iter().next());

    let graph = create_graph(&sorted_map);
}

fn get_all_five_words(file: File) -> Vec<String> {
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() == 5)
        .collect()
}

fn remove_repeat_letter_words(words: Vec<String>) -> Vec<String> {
    words.into_iter().filter(|w| word_diff_letters(w)).collect()
}

fn word_diff_letters(word: &String) -> bool {
    for i in 0..word.len() {
        let check_letter = word.chars().nth(i).unwrap();
        for j in i + 1..word.len() {
            let next_letter = word.chars().nth(j).unwrap();
            if check_letter == next_letter {
                return false;
            }
        }
    }
    true
}

fn sort_anagrams(words: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for word in words {
        let mut sorted_vec: Vec<char> = word.chars().collect();
        sorted_vec.sort_unstable();
        let sorted: String = sorted_vec.into_iter().collect();

        map.entry(sorted)
            .and_modify(|v| v.push(word.clone()))
            .or_insert(vec![word.clone()]);
    }
    map
}

fn create_graph<'a>(map: &'a HashMap<String, Vec<String>>) -> Vec<NodeGraph> {
    println!("Creating graph...");
    let start = Instant::now();
    let pb = ProgressBar::new(map.len().try_into().unwrap());

    let mut graph: Vec<NodeGraph> = Vec::new();

    let keys = Vec::from_iter(map.into_iter().map(|e| e.0));

    // Graph must be symmetric, because five word series is symmetric.
    // So to remove loops in graph we dont check words that was inserted into graph.
    for (i, k1) in keys.iter().enumerate() {
        let mut paths: Vec<&String> = Vec::new();
        for j in i + 1..keys.len() {
            let k2 = keys[j];
            if !check_if_words_cover(k1, k2) {
                paths.push(k2);
            }
        }
        let node = NodeGraph { node: k1, paths };
        graph.push(node);
        pb.inc(1);
    }

    pb.finish();
    println!("Done. Time elapsed: {:?}", start.elapsed());
    graph
}

fn check_if_words_cover(w1: &String, w2: &String) -> bool {
    for i in 0..w1.len() {
        let l1 = w1.chars().nth(i).unwrap();
        for j in 0..w2.len() {
            let l2 = w2.chars().nth(j).unwrap();
            if l1 == l2 {
                return true;
            }
        }
    }
    false
}

fn find_five_unique(graph: Vec<NodeGraph>) -> Vec<Vec<&String>> {
    let mut unique: Vec<Vec<&String>> = Vec::new();

    // let mut discovered: Ha

    unique
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_word_diff_letters() {
        assert_eq!(word_diff_letters(&"abcde".to_string()), true);
        assert_eq!(word_diff_letters(&"abcda".to_string()), false);
        assert_eq!(word_diff_letters(&"abbde".to_string()), false);
        assert_eq!(word_diff_letters(&"cbcde".to_string()), false);
        assert_eq!(word_diff_letters(&"adcde".to_string()), false);
    }
}
