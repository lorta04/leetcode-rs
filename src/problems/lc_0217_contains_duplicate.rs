use std::collections::HashSet;
use std::fs;

const _INPUT_FILE: &str = "input/lc_0217_contains_duplicate.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let splited_lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let mut parsed_lines: Vec<Vec<i32>> = splited_lines
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

    for line in &mut parsed_lines {
        println!("{:?}", _contains_duplicate_3(line.clone()));
    }
}

pub fn _contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();
    for num in nums {
        if seen.contains(&num) {
            return true;
        } else {
            seen.insert(num);
        }
    }

    false
}

pub fn _contains_duplicate_2(nums: Vec<i32>) -> bool {
    let mut seen: HashSet<i32> = HashSet::with_capacity(nums.len());
    for num in nums {
        if !seen.insert(num) {
            return true;
        }
    }
    false
}

pub fn _contains_duplicate_3(nums: Vec<i32>) -> bool {
    let mut nums = nums.clone();
    nums.sort_unstable();
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            return true;
        }
    }
    false
}
