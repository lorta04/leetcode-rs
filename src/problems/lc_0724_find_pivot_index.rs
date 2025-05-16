use std::fs;

const _INPUT_FILE: &str = "input/lc_0724_find_pivot_index.txt";

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
        println!("{:?}", _pivot_index_2(line.clone()));
    }
}

pub fn _pivot_index(nums: Vec<i32>) -> i32 {
    let total_sum: i32 = nums.iter().sum();

    for (i, num) in nums.iter().enumerate() {
        let pivot_sum = total_sum - num;
        if (pivot_sum) % 2 == 0 {
            if i == 0 && nums.iter().skip(0).sum::<i32>() == 0 {
                return 0;
            } else if i == nums.len() - 1 && nums.iter().take(nums.len() - 1).sum::<i32>() == 0 {
                return (nums.len() - 1) as i32;
            } else if nums.iter().take(i).sum::<i32>() == pivot_sum / 2 {
                return i as i32;
            } else {
            }
        }
    }

    -1
}

pub fn _pivot_index_2(nums: Vec<i32>) -> i32 {
    let total_sum: i32 = nums.iter().sum();
    let mut left_sum: i32 = 0;

    for (i, num) in nums.iter().enumerate() {
        let curr_sum = total_sum - num;
        if curr_sum % 2 == 0 && left_sum == curr_sum / 2 {
            return i as i32;
        } else {
            left_sum += num;
        }
    }

    -1
}
