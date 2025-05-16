use std::collections::HashSet;
use std::fs;

const _INPUT_FILE: &str = "input/lc_0202_happy_number.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let splited_lines: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    println!("{:?}", splited_lines);

    for line in splited_lines {
        println!("{}", _is_happy(line));
    }
}

pub fn _is_happy(n: i32) -> bool {
    let mut curr = n;
    let mut a: HashSet<i32> = HashSet::new();

    while !a.contains(&curr) {
        a.insert(curr);
        curr = _sum_of_squares(curr);
        if curr == 1 {
            return true;
        }
    }

    false
}

pub fn _sum_of_squares(mut n: i32) -> i32 {
    let mut sum = 0;
    while n != 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }

    sum
}
