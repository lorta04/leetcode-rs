use std::fs;

const _INPUT_FILE: &str = "input/lc_0383_ransom_note.txt";

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
            _can_construct(line.0.to_string(), line.1.to_string())
        );
    }
}

pub fn _can_construct(ransom_note: String, magazine: String) -> bool {
    let mut a: [i32; 26] = [0; 26];
    for c in magazine.chars() {
        a[(c as u8 - 'a' as u8) as usize] += 1
    }

    for c in ransom_note.chars() {
        if a[(c as u8 - 'a' as u8) as usize] == 0 {
            return false;
        }
        a[(c as u8 - 'a' as u8) as usize] -= 1
    }

    true
}
