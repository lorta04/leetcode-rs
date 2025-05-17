use std::{cmp::min, collections::HashMap, fs, i32::MAX};

const _INPUT_FILE: &str = "input/lc_0322_coin_change.txt";

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
        println!("{:?}", _coin_change_2(line.0.clone(), line.1.clone()));
    }
}

pub fn _coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut a: HashMap<i32, i32> = HashMap::new();
    a.insert(amount, 0);

    for amt in (0..amount).rev() {
        let mut min_coins = MAX;

        for coin in &coins {
            if let Some(loaded) = a.get(&(amt + *coin)) {
                min_coins = min(min_coins, loaded + 1);
            }
        }

        if min_coins != MAX {
            a.insert(amt, min_coins);
        }
    }

    *a.get(&0).unwrap_or(&-1)
}

pub fn _coin_change_2(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut a: Vec<i32> = vec![-1; (amount + 1) as usize];
    a[amount as usize] = 0;

    for amt in (0..amount).rev() {
        let mut min_coins = MAX;

        for coin in &coins {
            let loaded = *a.get((amt + *coin) as usize).unwrap_or(&-1);
            if loaded != -1 {
                min_coins = min(min_coins, loaded + 1);
            }
        }

        if min_coins != MAX {
            a[amt as usize] = min_coins;
        }
    }

    a[0]
}
