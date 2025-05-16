use std::{fs, vec};

const _INPUT_FILE: &str = "input/lc_0645_set_mismatch.txt";

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
        println!("{:?}", _find_error_nums(line));
    }
}

pub fn _find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut duplicate = 0;
    let mut missing = 0;
    let mut vec_nums = nums.clone();

    for num in nums {
        if vec_nums[(num - 1) as usize] < 0 {
            duplicate = num;
        } else {
            vec_nums[(num - 1) as usize] = -vec_nums[(num - 1) as usize];
        }
    }

    for (idx, num) in vec_nums.iter().enumerate() {
        if *num > 0 {
            missing = (idx as i32) + 1;
            break;
        }
    }

    vec![duplicate, missing]
}
