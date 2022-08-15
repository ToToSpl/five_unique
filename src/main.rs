use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Error, Write};
use std::time::Instant;

use indicatif::ProgressBar;

const WORD_FILENAME: &str = "words_5.txt";
const GRAPH_FILENAME: &str = "graph.csv";
const OUT_FILENAME: &str = "cliques.txt";

fn main() {
    let sorted_map = {
        let file = File::open(WORD_FILENAME).expect("Failed to open words file");
        let words_five_letter = get_all_five_words(file);
        println!("all five letter words: {}", words_five_letter.len());

        let words_five_letter = remove_repeat_letter_words(words_five_letter);
        println!("without repeating letters: {}", words_five_letter.len());

        let sorted_map = sort_anagrams(&words_five_letter);
        println!("with reduced anagrams: {}", sorted_map.len());
        sorted_map
    };

    let graph = match read_graph(GRAPH_FILENAME) {
        Ok(g) => g,
        Err(e) => {
            println!(
                "Failed to load the graph: {}\n. Creating new one...",
                e.to_string()
            );

            let graph = create_graph(&sorted_map);
            write_graph(&graph, GRAPH_FILENAME).expect("Failed to write graph to csv!");
            graph
        }
    };

    println!("Create optimized graph...");
    let optimized_graph = create_index_graph(&graph);

    let unique = find_five_unique(&optimized_graph);

    println!("Writing all cliques...");
    write_all_cliques_from_unique(&unique, &sorted_map, OUT_FILENAME)
        .expect("Failed to write output!");
}

fn write_all_cliques_from_unique(
    unique: &Vec<Vec<usize>>,
    sorted_map: &HashMap<String, Vec<String>>,
    filename: &str,
) -> std::io::Result<()> {
    let mut keys: Vec<&String> = sorted_map.keys().collect();
    keys.sort_unstable();
    let mut file = File::create(filename)?;

    for uniq_vec in unique {
        let sorted_anagrams: Vec<&String> = uniq_vec.iter().map(|k| keys[*k]).collect();
        let mut words: Vec<String> = Vec::new();
        recur_create_words(0, "".to_string(), &mut words, &sorted_anagrams, sorted_map);
        for word in words {
            file.write((word + "\n").as_bytes())?;
        }
    }

    Ok(())
}

fn recur_create_words(
    i: usize,
    input_str: String,
    input_vec: &mut Vec<String>,
    sorted_anagrams: &Vec<&String>,
    sorted_map: &HashMap<String, Vec<String>>,
) {
    if i == 5 {
        input_vec.push(input_str);
        return;
    }
    let ava_wrds = sorted_map.get(sorted_anagrams[i]).unwrap();
    for wrds in ava_wrds {
        let input_str_next = input_str.clone() + " " + wrds;
        recur_create_words(
            i + 1,
            input_str_next,
            input_vec,
            sorted_anagrams,
            sorted_map,
        )
    }
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

fn word_diff_letters(word: &str) -> bool {
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

fn create_index_graph(graph: &HashMap<String, Vec<String>>) -> Vec<Vec<usize>> {
    let mut keys = Vec::from_iter(graph.keys());
    keys.sort_unstable();

    let mut key_to_index = HashMap::new();
    for (i, k) in keys.iter().enumerate() {
        key_to_index.insert(k.clone(), i);
    }

    let mut out_vec = Vec::new();
    for k in keys {
        let paths = graph.get(k).unwrap();
        let mut index_paths = Vec::new();
        for p in paths {
            index_paths.push(key_to_index.get(p).unwrap().clone());
        }
        out_vec.push(index_paths);
    }

    out_vec
}

fn create_graph(map: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    println!("Creating graph...");
    let start = Instant::now();
    let pb = ProgressBar::new(map.len().try_into().unwrap());

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut keys = Vec::from_iter(map.iter().map(|e| e.0));
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

#[inline(always)]
fn check_if_words_cover(w1: &str, w2: &str) -> bool {
    for l1 in w1.chars() {
        for l2 in w2.chars() {
            if l1 == l2 {
                return true;
            }
        }
    }
    false
}

#[inline(always)]
fn compare_two_paths<'a>(p1: &'a [usize], p2: &'a [usize]) -> Vec<usize> {
    let mut out = Vec::new();
    for a in p2 {
        for b in p1 {
            if a == b {
                out.push(*a);
            }
        }
    }
    out
}

fn find_five_unique(graph: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut unique: Vec<Vec<usize>> = Vec::new();
    let start = Instant::now();
    let pb = ProgressBar::new(graph.len().try_into().unwrap());
    println!("Finding five unique in graph...");
    let mut unique_num = 0;

    for (i, n_i) in graph.iter().enumerate() {
        pb.inc(1);
        if n_i.len() < 4 {
            continue;
        }
        for j in n_i {
            let r_ij = &graph[*j];
            if r_ij.len() < 3 {
                continue;
            }
            let n_ij = compare_two_paths(&n_i, &r_ij);
            if n_ij.len() < 3 {
                continue;
            }
            for k in &n_ij {
                let r_ijk = &graph[*k];
                if r_ijk.len() < 2 {
                    continue;
                }
                let n_ijk = compare_two_paths(&n_ij, &r_ijk);
                if n_ijk.len() < 2 {
                    continue;
                }
                for l in &n_ijk {
                    let r_ijkl = &graph[*l];
                    let n_ijkl = compare_two_paths(&n_ijk, &r_ijkl);
                    for m in &n_ijkl {
                        unique.push(vec![
                            i.clone(),
                            (*j).clone(),
                            (*k).clone(),
                            (*l).clone(),
                            (*m).clone(),
                        ]);
                        unique_num += 1;
                    }
                }
            }
        }
    }

    pb.finish();
    println!("Done. Time elapsed: {:?}", start.elapsed());
    println!("Found unique: {}", unique_num);
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

    #[test]
    fn test_check_if_words_cover() {
        assert_eq!(
            check_if_words_cover(&("abcd".to_string()), &("aefgh".to_string())),
            true
        );
        assert_eq!(
            check_if_words_cover(&("abcd".to_string()), &("efghi".to_string())),
            false
        );
        assert_eq!(
            check_if_words_cover(&("abcd".to_string()), &("abcd".to_string())),
            true
        );
        assert_eq!(
            check_if_words_cover(&("abcd".to_string()), &("abcde".to_string())),
            true
        );
        assert_eq!(
            check_if_words_cover(&("abcd".to_string()), &("xyzvw".to_string())),
            false
        );
    }
}
