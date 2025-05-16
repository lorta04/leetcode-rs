use std::{cmp::max, fs, i32::MIN};

const _INPUT_FILE: &str = "input/lc_0053_maximum_subarray.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let splited_lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let parsed_lines: Vec<Vec<i32>> = splited_lines
        .iter()
        .map(|line| {
            line.trim_matches(|c| c == '[' || c == ']') // Remove [ and ]
                .split(',') // Split by comma
                .filter(|s| !s.trim().is_empty()) // Skip empty entries
                .map(|num| num.trim().parse::<i32>().unwrap()) // Parse i32
                .collect()
        })
        .collect();

    println!("{:?}", parsed_lines);

    for line in parsed_lines {
        println!("{:?}", _max_sub_array(line));
    }
}

pub fn _max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sub = MIN;
    let mut curr_sub = 0;

    for num in nums {
        if curr_sub < 0 {
            curr_sub = 0
        }
        curr_sub += num;
        max_sub = max(max_sub, curr_sub)
    }

    max_sub
}
