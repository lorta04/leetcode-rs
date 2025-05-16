use std::collections::HashMap;
use std::fs;

const _INPUT_FILE: &str = "input/lc_0350_intersection_of_two_arrays_ii.txt";

fn _parse_vec(line: &str) -> Vec<i32> {
    line.trim_matches(['[', ']'].as_ref())
        .split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect()
}

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");

    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut parsed: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();

    for chunk in lines.chunks(2) {
        if chunk.len() < 2 {
            continue; // skip if an incomplete pair
        }

        let vec1 = _parse_vec(chunk[0]);
        let vec2 = _parse_vec(chunk[1]);

        parsed.push((vec1, vec2));
    }

    println!("{:?}", parsed);

    for line in parsed {
        println!("{:?}", _intersect(line.0.clone(), line.1.clone()));
    }
}

pub fn _intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut a: HashMap<i32, i32> = HashMap::new();

    for num in nums1 {
        a.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut res: Vec<i32> = vec![];

    for num in nums2 {
        if let Some(count) = a.get_mut(&num) {
            *count -= 1;
            res.push(num);
            if *count == 0 {
                a.remove(&num);
            }
        }
    }

    res
}
