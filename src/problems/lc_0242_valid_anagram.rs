use std::fs;

const _INPUT_FILE: &str = "input/lc_0242_valid_anagram.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let lines: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.trim().split(" ").collect())
        .map(|pair_str: Vec<&str>| (pair_str[0], pair_str[1]))
        .collect();

    println!("{:?}", lines);

    for line in lines {
        println!("{:?}", _is_anagram(line.0.to_string(), line.1.to_string()));
    }
}

pub fn _is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut arr: [i32; 26] = [0; 26];

    for c in s.chars() {
        arr[(c as u8 - 97) as usize] += 1;
    }

    for c in t.chars() {
        arr[(c as u8 - 97) as usize] -= 1;
    }

    arr == [0; 26]
}

pub fn _is_anagram_2(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut arr = [0; 26];

    for (sc, tc) in s.bytes().zip(t.bytes()) {
        // Assume lowercase a-z; subtract ASCII 'a' (97)
        arr[(sc - b'a') as usize] += 1;
        arr[(tc - b'a') as usize] -= 1;
    }

    arr.iter().all(|&count| count == 0)
}
