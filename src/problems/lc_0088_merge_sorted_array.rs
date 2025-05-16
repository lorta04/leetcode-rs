use std::fs;

const _INPUT_FILE: &str = "input/lc_0088_merge_sorted_array.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let lines: Vec<&str> = input.lines().collect();

    let mut nums1: Vec<i32> = lines[0]
        .trim_matches(&['[', ']'][..])
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let m: i32 = lines[1].trim().parse().unwrap();
    let mut nums2: Vec<i32> = lines[2]
        .trim_matches(&['[', ']'][..])
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let n: i32 = lines[3].trim().parse().unwrap();

    _merge(&mut nums1, m, &mut nums2, n);

    println!("nums1: {:?}, m: {}", nums1, m);
    println!("nums2: {:?}, n: {}", nums2, n);
}

pub fn _merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if m == 0 {
        for i in 0..n {
            nums1[i as usize] = nums2[i as usize];
        }
    }

    if n == 0 {
        return;
    }

    for i in (0..m).rev() {
        nums1[(n + i) as usize] = nums1[i as usize];
    }

    let mut curr: usize = 0;
    let mut l: i32 = 0;
    let mut r: i32 = 0;
    while (l < m) && (r < n) {
        if nums1[(l + n) as usize] < nums2[r as usize] {
            nums1.swap(curr, (l + n) as usize);
            l += 1;
        } else {
            nums1[curr] = nums2[r as usize];
            r += 1;
        }
        curr += 1;
    }

    while l < m {
        nums1.swap(curr, (l + n) as usize);
        l += 1;
        curr += 1;
    }

    while r < n {
        nums1[curr] = nums2[r as usize];
        r += 1;
        curr += 1;
    }
}
