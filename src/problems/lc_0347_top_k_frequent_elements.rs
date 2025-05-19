use std::{collections::HashMap, fs, i32::MIN};

const _INPUT_FILE: &str = "input/lc_0347_top_k_frequent_elements.txt";

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

    for line in result {
        println!("{:?}", _top_k_frequent(line.0, line.1));
    }
}

pub fn _top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count_by_num: HashMap<i32, i32> = HashMap::new();
    let mut nums_by_count: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];
    let mut res: Vec<i32> = Vec::with_capacity(k as usize);

    for num in nums {
        count_by_num.entry(num).and_modify(|v| *v += 1).or_insert(1);
    }

    for (key, val) in count_by_num {
        nums_by_count[(val - 1) as usize].push(key);
    }

    let mut k = k;
    let mut idx: i32 = (nums_by_count.len() - 1) as i32;
    while k > 0 && idx >= 0 {
        let nums = &nums_by_count[idx as usize];
        for num in nums {
            res.push(*num);
            k -= 1;
        }

        idx -= 1;
    }

    res
}
