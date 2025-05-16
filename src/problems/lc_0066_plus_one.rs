use std::{fs, vec};

const _INPUT_FILE: &str = "input/lc_0066_plus_one.txt";

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
        println!("{:?}", _plus_one(line.clone()));
    }
}

pub fn _plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = digits.clone();
    let mut is_carry = false;
    let last_index = res.len() - 1;

    if res[last_index] == 9 {
        if res.len() == 1 {
            // edge case
            return vec![1, 0];
        }

        is_carry = true;
        res[last_index] = 0;
    } else {
        res[last_index] = res[last_index] + 1;
    }

    for i in (0..(res.len() - 1)).rev() {
        if is_carry {
            if res[i] == 9 {
                res[i] = 0;
                if i == 0 {
                    res.insert(0, 1);
                }
            } else {
                is_carry = false;
                res[i] = res[i] + 1;
            }
        } else {
            break;
        }
    }

    res
}

pub fn _plus_one_2(digits: Vec<i32>) -> Vec<i32> {
    let mut res = digits.clone();
    let mut i = res.len() - 1;

    // Add 1 to the last digit
    res[i] += 1;

    // Propagate carry if needed
    while res[i] == 10 {
        res[i] = 0;
        if i == 0 {
            res.insert(0, 1);
            break;
        } else {
            i -= 1;
            res[i] += 1;
        }
    }

    res
}
