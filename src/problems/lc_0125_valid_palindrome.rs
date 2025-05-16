use std::fs;

const _INPUT_FILE: &str = "input/lc_0125_valid_palindrome.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let splited_lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();

    println!("{:?}", splited_lines);

    for line in &splited_lines {
        if _is_palindrome_2(line) {
            println!("true");
        } else {
            println!("false");
        }
    }
}

pub fn _is_palindrome(s: &String) -> bool {
    let cleaned_s = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();

    for idx in 0..(cleaned_s.len() / 2) {
        let left = cleaned_s.chars().nth(idx).unwrap();
        let right = cleaned_s
            .chars()
            .nth(cleaned_s.chars().count() - 1 - idx)
            .unwrap();

        if left != right {
            return false;
        }
    }

    true
}

pub fn _is_palindrome_2(s: &String) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}
