extern crate time;

use std::collections::HashSet;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use time::precise_time_ns;

pub struct Dictionary{
    dict: HashSet<String>,
}

impl Dictionary {
    /// Makes a new 'Dictionary' by reading words from a file, 1 per line.
    pub fn load_from_file(path: &Path) -> Dictionary {
        let mut dict = HashSet::new();
        let file = BufReader::new(File::open(path).unwrap());
        for line in file.lines() {
            dict.insert(line.unwrap().trim().to_string());
        }
        Dictionary {dict: dict}
    }

    /// Finds words of length 6 made up of two words 2 or more characters long.
    pub fn words_of_two(&self) -> Vec<String> {
        let mut matches: Vec<String> = Vec::new();
        for word in self.dict.iter() {
            if word.len() != 6 { continue  }
            for i in 2..5 {
                if self.dict.contains(&word[..i]) &&
                       self.dict.contains(&word[i..]) {
                    matches.push(word.clone());
                    break
                }
            }
        }
        matches
    }
}

pub fn main() {
    println!("Started...");
    let start = precise_time_ns();
    let path = Path::new("assets/wordsEn.txt");
    let dict = Dictionary::load_from_file(&path);
    let loaded = precise_time_ns();
    let words = dict.words_of_two();
    let search = precise_time_ns();
    println!("Found:  {} words" , words.len());
    println!("Load:   {} ns" , loaded - start);
    println!("Search: {} ns" , search - loaded);
    println!("Total:  {} ns" , search - start);
}

#[cfg(test)]
mod test_bench {
    use std::path::Path;
    use std::collections::HashSet;
    use Dictionary;

    #[test]
    fn should_find_a_word_of_two() {
        let mut set = HashSet::new();
        set.insert("hub".to_string());
        set.insert("bub".to_string());
        set.insert("hubbub".to_string());
        let dict = Dictionary {dict: set};

        assert_eq!("hubbub" , dict.words_of_two()[0]);
    }

    #[test]
    fn should_find_n_words_of_two() {
        let path = Path::new("assets/wordsEn.txt");
        let dict = Dictionary::load_from_file(&path);
        assert_eq!(3715, dict.words_of_two().len());
    }


    // Unstable benchmark.
    //use std::io::prelude::*;
    //use std::io::BufReader;
    //use std::fs::File;
    //#[bench]
    //fn bench_words_of_two(b: &mut Bencher) {
    //    let path = Path::new("assets/wordsEn.txt");
    //    let dict = Dictionary::load_from_file(&path);
    //    b.iter(|| dict.words_of_two());
    //}
    //
    //#[bench]
    //fn bench_load_dictionary(b: &mut Bencher) {
    //    let path = Path::new("assets/wordsEn.txt");
    //    b.iter(|| Dictionary::load_from_file(&path));
    //}
}
