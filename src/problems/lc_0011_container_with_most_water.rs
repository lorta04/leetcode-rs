use std::{cmp::max, fs};

const _INPUT_FILE: &str = "input/lc_0011_container_with_most_water.txt";

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
        println!("{:?}", _max_area(line));
    }
}

pub fn _max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_area = 0;

    while left < right {
        let width = right - left;
        let min_height;

        if height[left] <= height[right] {
            min_height = height[left];
            left += 1;
        } else {
            min_height = height[right];
            right -= 1
        }

        max_area = max(max_area, width as i32 * min_height);
    }

    max_area
}
