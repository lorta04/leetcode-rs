use std::{cmp::min, fs, i32::MAX};

const _INPUT_FILE: &str = "input/lc_0209_minimum_size_subarray_sum.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    let mut result = vec![];

    let mut i = 0;
    while i + 1 < lines.len() {
        let nums = lines[i]
            .trim_matches(['[', ']'].as_ref())
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let target = lines[i + 1].parse::<i32>().unwrap();
        result.push((nums, target));
        i += 2;
    }

    println!("{:?}", result);

    for line in &mut result {
        println!("{:?}", _min_sub_array_len(line.1.clone(), line.0.clone()));
    }
}

pub fn _min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut sum = nums[0];
    let mut min_length: Option<i32> = None;

    while right < nums.len() {
        if sum < target {
            right += 1;
            if let Some(num) = nums.get(right) {
                sum += num;
            };
        } else {
            min_length = Some(min(min_length.unwrap_or(MAX), (right - left + 1) as i32));

            if left == right {
                left += 1;
                right += 1;
                if let Some(num) = nums.get(right) {
                    sum = *num;
                };
            } else {
                sum -= nums[left];
                left += 1;
            }
        }
    }

    min_length.unwrap_or(0)
}
