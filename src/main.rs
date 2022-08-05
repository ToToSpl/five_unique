use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("words_alpha.txt").expect("Failed to open words_alpha file");
    let words_five_letter = get_all_five_words(file);
    println!("all five letter words: {}", words_five_letter.len());

    let words_five_letter = remove_repeat_letter_words(words_five_letter);
    println!("without repeating letters: {}", words_five_letter.len());


    for i in (0..words_five_letter.len()).step_by(words_five_letter.len() / 20) {
        println!("{:}", words_five_letter[i]);
    }

    println!("Hello, world!");
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
