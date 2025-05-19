use std::fs;

const _INPUT_FILE: &str = "input/lc_0238_product_of_array_except_self.txt";

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
        println!("{:?}", _product_except_self(line));
    }
}

pub fn _product_except_self(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 1 {
        return nums;
    }

    let mut res = vec![1; nums.len()];

    let mut product = 1;
    for i in 1..nums.len() {
        product *= nums[i - 1];
        res[i] = product;
    }

    let mut right_product = 1;
    for i in (0..nums.len()).rev() {
        res[i] *= right_product;
        right_product *= nums[i];
    }

    res
}
