use std::collections::HashSet;
use std::fs;

const _INPUT_FILE: &str = "input/lc_0003_longest_substring_without_repeating_characters.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let splited_lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    println!("{:?}", splited_lines);

    for line in &splited_lines {
        println!("{}", _length_of_longest_substring_2(line));
    }
}

pub fn _length_of_longest_substring(s: &String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    let vec_s: Vec<char> = s.chars().collect();
    let mut seen: HashSet<char> = HashSet::new();

    let mut left = 0;
    let mut max_len = 0;

    for right in 0..vec_s.len() {
        while seen.contains(&vec_s[right]) {
            seen.remove(&vec_s[left]);
            left += 1;
        }

        seen.insert(vec_s[right]);
        max_len = max_len.max(right - left + 1);
    }

    max_len as i32
}

pub fn _length_of_longest_substring_2(s: &String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    let vec_s = s.as_bytes();
    let mut seen: HashSet<u8> = HashSet::new();

    let mut left = 0;
    let mut max_len = 0;

    for right in 0..vec_s.len() {
        while seen.contains(&vec_s[right]) {
            seen.remove(&vec_s[left]);
            left += 1;
        }

        seen.insert(vec_s[right]);
        max_len = max_len.max(right - left + 1);
    }

    max_len as i32
}
