use std::collections::HashMap;
use std::fs;

const _INPUT_FILE: &str = "input/lc_0205_isomorphic_strings.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let lines: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.trim().split(" ").collect())
        .map(|pair_str: Vec<&str>| (pair_str[0], pair_str[1]))
        .collect();

    println!("{:?}", lines);

    for line in lines {
        println!(
            "{:?}",
            _is_isomorphic(line.0.to_string(), line.1.to_string())
        );
    }
}

pub fn _is_isomorphic(s: String, t: String) -> bool {
    let mut a: HashMap<char, char> = HashMap::new();
    let mut b: HashMap<char, char> = HashMap::new();

    for (c1, c2) in s.chars().zip(t.chars()) {
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
