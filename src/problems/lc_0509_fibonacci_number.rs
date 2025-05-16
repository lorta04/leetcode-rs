use std::fs;

const _INPUT_FILE: &str = "input/lc_0509_fibonacci_number.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let splited_lines: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    println!("{:?}", splited_lines);

    for line in splited_lines {
        println!("{}", _fib(line));
    }
}

pub fn _fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }

    let mut left = 1;
    let mut right = 1;
    let mut res = 0;

    for _ in 3..n + 1 {
        res = left + right;
        left = right;
        right = res;
    }

    res
}
