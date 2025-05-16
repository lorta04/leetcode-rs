use std::fs;

const _INPUT_FILE: &str = "input/lc_0387_first_unique_character_in_a_string.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let splited_lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    println!("{:?}", splited_lines);

    for line in splited_lines {
        println!("{}", _first_uniq_char(line));
    }
}

pub fn _first_uniq_char(s: String) -> i32 {
    let mut a: [i32; 26] = [0; 26];
    for c in s.chars() {
        a[(c as u8 - 'a' as u8) as usize] += 1
    }

    for (i, c) in s.chars().enumerate() {
        if a[(c as u8 - 'a' as u8) as usize] == 1 {
            return i as i32;
        }
    }

    -1
}
