extern crate test;

use std::collections::HashSet;
use std::io::BufferedReader;
use std::io::File;
use test::Bencher;


fn main() {
    let path = Path::new("assets/wordsEn.txt");
    let dict = load_dictionary(path);
    let matches = find(dict);
    println!("{}", matches);
}

fn load_dictionary(path: Path) -> HashSet<String> {
    let mut dict = HashSet::new();
    let mut file = BufferedReader::new(File::open(&path));
    for line in file.lines() {
        dict.insert(line.unwrap().trim().to_string());
    }
    dict
}

fn find(dict: HashSet<String>) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();
    for word in dict.iter() {
        if (word.len() != 6) { continue }
        for i in range(2u, 5) {
          if dict.contains(word.slice_to(i)) && dict.contains(word.slice_from(i)) {
            matches.push(word.clone());
            break
          }
        }
    }
    matches
}

fn foo(bar: int) -> int {
  bar + 1
}

#[bench]
fn bench_find(b: &mut Bencher) {
    let bar = 1i;
    b.iter(|| foo(bar));
}
