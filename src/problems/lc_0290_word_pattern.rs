use std::collections::HashMap;
use std::fs;

const _INPUT_FILE: &str = "input/lc_0290_word_pattern.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let lines: Vec<&str> = input.lines().map(|line| line.trim_matches('"')).collect();

    let mut parsed: Vec<(String, Vec<String>)> = Vec::new();

    for i in (0..lines.len()).step_by(2) {
        let pattern = lines[i].to_string();
        let words = lines[i + 1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        parsed.push((pattern, words));
    }

    println!("{:?}", parsed);

    for (pattern, words) in parsed {
        println!("{:?}", _word_pattern(pattern, words.join(" ")));
    }
}

pub fn _word_pattern(pattern: String, s: String) -> bool {
    let vec_s: Vec<&str> = s.split_whitespace().collect();

    if pattern.len() != vec_s.len() {
        return false;
    }

    let mut a: HashMap<char, &str> = HashMap::new();
    let mut b: HashMap<&str, char> = HashMap::new();

    for (c1, c2) in pattern.chars().zip(vec_s) {
        if !a.contains_key(&c1) {
            if b.contains_key(&c2) {
                return false;
            }
            a.insert(c1, c2);
            b.insert(c2, c1);
        } else {
            if a.get(&c1).unwrap() != &c2 {
                return false;
            }
        }
    }

    true
}
