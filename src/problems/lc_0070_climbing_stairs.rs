use std::fs;

const _INPUT_FILE: &str = "input/lc_0070_climbing_stairs.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let splited_lines: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    println!("{:?}", splited_lines);

    for line in splited_lines {
        println!("{}", _climb_stairs(line));
    }
}

pub fn _climb_stairs(n: i32) -> i32 {
    let mut a = vec![0; (n + 1) as usize];
    a[0] = 1;

    for step in 0..n + 1 {
        if step + 1 <= n {
            a[(step + 1) as usize] += a[step as usize];
        }
        if step + 2 <= n {
            a[(step + 2) as usize] += a[step as usize];
        }
    }

    a[n as usize]
}
