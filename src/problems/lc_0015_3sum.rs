use std::fs;

const _INPUT_FILE: &str = "input/lc_0015_3sum.txt";

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
        println!("{:?}", _three_sum(line));
    }
}

// 3 optimization trick
// 1. limit two pointers' search range to only the right side of the main index
// 2. skip duplicate values for main index when moving from left to right
// 3. skip duplicate values when moveing two pointers in when a result is found

pub fn _three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut res: Vec<Vec<i32>> = vec![];

    for (main_idx, num) in nums.iter().take(nums.len() - 2).enumerate() {
        if main_idx != 0 && *num == nums[main_idx - 1] {
            continue;
        }

        let mut left = main_idx + 1;
        let mut right = nums.len() - 1;
        let target = -num;

        while left < right {
            let sum = nums[left] + nums[right];

            if sum == target {
                res.push(vec![*num, nums[left], nums[right]]);

                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }

                left += 1;
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    res
}
