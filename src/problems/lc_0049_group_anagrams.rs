use std::collections::HashMap;
use std::fs;

const _INPUT_FILE: &str = "input/lc_0049_group_anagrams.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let splited_lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let parsed_lines: Vec<Vec<String>> = splited_lines
        .iter()
        .map(|line| {
            line.trim_matches(|c| c == '[' || c == ']') // Remove [ and ]
                .split(',') // Split by comma
                .map(|s| {
                    s.trim_matches(|c: char| c == '"' || c.is_whitespace())
                        .to_string()
                })
                .collect()
        })
        .collect();

    println!("{:?}", parsed_lines);

    for line in parsed_lines {
        println!("{:?}", _group_anagrams(line));
    }
}

pub fn _group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut a: HashMap<[i32; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut counts: [i32; 26] = [0; 26];

        for c in s.chars() {
            counts[(c as u8 - 97) as usize] += 1;
        }

        a.entry(counts)
            .and_modify(|val: &mut Vec<String>| val.push(s.clone()))
            .or_insert(vec![s.clone()]);
    }

    let mut result: Vec<Vec<String>> = Vec::new();
    for values in a.values() {
        result.push(values.clone());
    }

    result
}
