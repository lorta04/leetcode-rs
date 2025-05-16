use std::fs;

const _INPUT_FILE: &str = "input/lc_0283_move_zeroes.txt";

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
        _move_zeroes(line);
        println!("{:?}", line);
    }
}

pub fn _move_zeroes(nums: &mut Vec<i32>) {
    let mut l = 0;
    for r in 0..nums.len() {
        if nums[r] != 0 {
            nums.swap(l, r);
            l += 1;
        }
    }
}
