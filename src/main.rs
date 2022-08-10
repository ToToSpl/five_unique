use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Error, Write};
use std::time::Instant;

use indicatif::ProgressBar;

fn main() {
    let graph = match read_graph("graph.csv") {
        Ok(g) => g,
        Err(_) => {
            println!("Failed to load the graph. Creating new one...");
            let file = File::open("words_alpha.txt").expect("Failed to open words_alpha file");
            let words_five_letter = get_all_five_words(file);
            println!("all five letter words: {}", words_five_letter.len());

            let words_five_letter = remove_repeat_letter_words(words_five_letter);
            println!("without repeating letters: {}", words_five_letter.len());

            let sorted_map = sort_anagrams(&words_five_letter);
            println!("with reduced anagrams: {}", sorted_map.len());

            let graph = create_graph(&sorted_map);
            write_graph(&graph, "graph.csv").unwrap();
            graph
        }
    };

    // let unique = find_five_unique(&graph);

    // println!("{:?}", unique);
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

fn create_graph<'a>(map: &'a HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    println!("Creating graph...");
    let start = Instant::now();
    let pb = ProgressBar::new(map.len().try_into().unwrap());

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut keys = Vec::from_iter(map.into_iter().map(|e| e.0));
    keys.sort_unstable();

    // Graph must be symmetric, because five word series is symmetric.
    // So to remove loops in graph we dont check words that was inserted previously into graph.
    for (i, k1) in keys.iter().enumerate() {
        let mut paths: Vec<&String> = Vec::new();
        for j in i + 1..keys.len() {
            let k2 = keys[j];
            if !check_if_words_cover(k1, k2) {
                paths.push(k2);
            }
        }
        graph.insert(
            (*k1).clone(),
            paths.into_iter().map(|s| s.clone()).collect(),
        );
        pb.inc(1);
    }

    pb.finish();
    println!("Done. Time elapsed: {:?}", start.elapsed());
    graph
}

fn write_graph(graph: &HashMap<String, Vec<String>>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for (k, v) in graph.clone().into_iter() {
        let mut line = String::new();
        line += k.as_str();
        for s in v {
            line += format!(" {}", s).as_str();
        }
        line += "\n";
        file.write(line.as_bytes())?;
    }
    Ok(())
}

fn read_graph(filename: &str) -> Result<HashMap<String, Vec<String>>, Error> {
    let file = File::open(filename)?;
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|f| f.split(' ').map(|s| String::from(s)).collect())
        .for_each(|v: Vec<String>| {
            graph.insert(v[0].clone(), v[1..].to_vec());
        });
    Ok(graph)
}

fn pointer_graph<'a>(
    graph: &'a HashMap<String, Vec<String>>,
) -> HashMap<&'a String, Vec<&'a String>> {
    let mut g = HashMap::new();
    for (k, v) in graph {
        let mut new_v = Vec::new();
        for ev in v {
            new_v.push(ev);
        }
        g.insert(k, new_v);
    }
    g
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

fn compare_two_paths<'a>(p1: &'a Vec<String>, p2: &'a Vec<String>) -> Vec<&'a String> {
    let mut out = Vec::new();
    for a in p1 {
        for b in p2 {
            if a == b {
                out.push(a);
            }
        }
    }
    out
}

fn find_five_unique<>(graph: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let start = Instant::now();

    let mut unique: Vec<Vec<String>> = Vec::new();
    let mut keys = Vec::from_iter(graph.iter().map(|e| e.0.clone()));
    keys.sort_unstable();

    let pb = ProgressBar::new(keys.len().try_into().unwrap());
    println!("Finding five unique in graph...");

    for key in keys {
        let node_p1 = graph.get(&key).unwrap();
        if node_p1.len() < 4 {
            continue;
        }
        for p1 in node_p1 {
            let node_p2 = graph.get(p1).unwrap();
            if node_p2.len() < 3 {
                continue;
            }
            let cnode_p2 = compare_two_paths(node_p1, node_p2);
            if cnode_p2.len() < 3 {
                continue;
            }
            for p2 in &cnode_p2 {
                let node_p3 = graph.get(p2).unwrap();
                if node_p3.len() < 2 {
                    continue;
                }
                let cnode_p3 = compare_two_paths(&cnode_p2, node_p3);
                if cnode_p3.len() < 2 {
                    continue;
                }
                for p3 in &cnode_p3 {
                    let node_p4 = graph.get(p3).unwrap();
                    if node_p4.len() < 1 {
                        continue;
                    }
                    let cnode_p4 = compare_two_paths(&cnode_p3, node_p4);
                    for p4 in &cnode_p4 {
                        unique.push(vec![
                            key.clone(),
                            (*p1).clone(),
                            (*p2).clone(),
                            (*p3).clone(),
                            (*p4).clone(),
                        ]);
                    }
                }
            }
        }

        pb.inc(1);
    }

    pb.finish();
    println!("Done. Time elapsed: {:?}", start.elapsed());
    unique
}

#[cfg(test)]
mod tests {
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
