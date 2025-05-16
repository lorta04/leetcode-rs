use std::cmp::max;
use std::fs;

const _INPUT_FILE: &str = "input/lc_0198_house_robber.txt";

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
        println!("{:?}", _rob(line.clone()));
    }
}

pub fn _rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut a = vec![0; nums.len()];
    a[0] = nums[0];
    a[1] = nums[1];

    for n in 2..nums.len() {
        let x = a[n - 2] + nums[n];
        let mut y = 0;
        if n >= 3 {
            y = a[n - 3] + nums[n];
        }
        a[n] = max(x, y);
    }

    let mut max_money = 0;
    for n in a {
        max_money = max(max_money, n);
    }
    max_money
}
