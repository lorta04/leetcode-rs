use std::fs;

const _INPUT_FILE: &str = "input/lc_0169_majority_element.txt";

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
        println!("{:?}", _majority_element(line.clone()));
    }
}

pub fn _majority_element(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut count = 0;
    for num in nums {
        if count == 0 {
            result = num
        }

        if num == result {
            count += 1;
        } else {
            count -= 1;
        }
    }

    result
}
