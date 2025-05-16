use std::collections::HashMap;
use std::fs;

const _INPUT_FILE: &str = "input/lc_167_two_sum_ii.txt";

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
        println!("{:?}", _two_sum_2(line.0.clone(), line.1.clone()));
    }
}

// invalid
// Your solution must use only constant extra space.
pub fn _two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut a: HashMap<i32, usize> = HashMap::new();

    for (i, num) in numbers.iter().enumerate() {
        if a.contains_key(num) {
            return vec![a.get(num).unwrap().clone() as i32, i as i32];
        } else {
            a.insert(target - num, i);
        }
    }

    vec![0, 0]
}

pub fn _two_sum_2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l: usize = 0;
    let mut r: usize = numbers.len() - 1;

    while l < r {
        let sum = numbers[l] + numbers[r];
        if sum < target {
            l += 1;
        } else if sum > target {
            r -= 1;
        } else {
            return vec![(l + 1) as i32, (r + 1) as i32];
        }
    }

    vec![0, 0]
}
