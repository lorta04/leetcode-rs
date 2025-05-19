use std::{collections::HashSet, fs, path::Path};

const _INPUT_FILE: &str = "input/lc_0210_course_schedule_ii.txt";

pub fn _run() {
    let input = match fs::read_to_string(Path::new(_INPUT_FILE)) {
        Ok(s) => s,
        Err(_) => {
            println!("[]");
            return;
        }
    };

    let lines: Vec<&str> = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    let mut result: Vec<(i32, Vec<Vec<i32>>)> = vec![];
    let mut i = 0;

    while i + 1 < lines.len() {
        let number = match lines[i].parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                i += 2;
                continue; // skip this pair
            }
        };

        let matrix = {
            let matrix_line = lines[i + 1].trim_matches(|c| c == '[' || c == ']');
            let parsed = matrix_line
                .split("],[")
                .filter(|s| !s.trim().is_empty())
                .map(|row| {
                    row.split(',')
                        .filter(|s| !s.trim().is_empty())
                        .map(|n| n.trim().parse::<i32>())
                        .collect::<Result<Vec<i32>, _>>()
                })
                .collect::<Result<Vec<Vec<i32>>, _>>();

            match parsed {
                Ok(mat) => mat,
                Err(_) => {
                    i += 2;
                    continue; // skip this pair
                }
            }
        };

        result.push((number, matrix));
        i += 2;
    }

    println!("{:?}", result);

    for (num, mat) in result {
        println!("{:?}", _find_order(num, mat));
    }
}

pub fn _find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut cleared: Vec<bool> = vec![false; num_courses as usize];
    let mut visiting: Vec<bool> = vec![false; num_courses as usize];
    let mut order: Vec<i32> = vec![];

    let mut prerequisites_by_course: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
    for prereq in prerequisites {
        prerequisites_by_course[prereq[0] as usize].push(prereq[1]);
    }

    for course in 0..num_courses {
        if !_find_order_dfs(
            course,
            &prerequisites_by_course,
            &mut cleared,
            &mut visiting,
            &mut order,
        ) {
            return vec![];
        }
    }

    order
}

pub fn _find_order_dfs(
    course: i32,
    prerequisites: &Vec<Vec<i32>>,
    cleared: &mut Vec<bool>,
    visiting: &mut Vec<bool>,
    order: &mut Vec<i32>,
) -> bool {
    if cleared[course as usize] {
        return true;
    }
    if visiting[course as usize] {
        return false;
    }

    visiting[course as usize] = true;
    for prereq in &prerequisites[course as usize] {
        if !_find_order_dfs(*prereq, prerequisites, cleared, visiting, order) {
            return false;
        }
    }
    visiting[course as usize] = false;

    cleared[course as usize] = true;
    order.push(course);
    true
}
