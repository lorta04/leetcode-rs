use std::{collections::HashSet, fs, path::Path};

const _INPUT_FILE: &str = "input/lc_0207_course_schedule.txt";

pub fn _run() {
    let input = fs::read_to_string(Path::new(_INPUT_FILE)).expect("Input file not found");
    let lines: Vec<&str> = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    let mut result: Vec<(i32, Vec<Vec<i32>>)> = vec![];
    let mut i = 0;

    while i + 1 < lines.len() {
        let number = lines[i].parse::<i32>().expect("Expected a number");

        let matrix = lines[i + 1]
            .trim_matches(|c| c == '[' || c == ']')
            .split("],[")
            .map(|row| {
                row.split(',')
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        result.push((number, matrix));
        i += 2;
    }

    println!("{:?}", result);

    for line in result {
        println!("{:?}", _can_finish(line.0, line.1));
    }
}

pub fn _can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut cleared: HashSet<i32> = HashSet::new();
    let mut visiting: HashSet<i32> = HashSet::new();

    let mut prerequisites_by_course: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
    for prereq in prerequisites {
        prerequisites_by_course[prereq[0] as usize].push(prereq[1]);
    }

    for course in 0..num_courses {
        if !_can_finish_course(
            course,
            &prerequisites_by_course,
            &mut cleared,
            &mut visiting,
        ) {
            return false;
        }
    }

    true
}

pub fn _can_finish_course(
    course: i32,
    prerequisites: &Vec<Vec<i32>>,
    cleared: &mut HashSet<i32>,
    visiting: &mut HashSet<i32>,
) -> bool {
    if cleared.contains(&course) {
        return true;
    }
    if visiting.contains(&course) {
        return false;
    }

    let mut prereq_passed = 0;

    visiting.insert(course);
    for prereq in &prerequisites[course as usize] {
        if _can_finish_course(*prereq, prerequisites, cleared, visiting) {
            prereq_passed += 1;
        } else {
            return false;
        }
    }
    visiting.remove(&course);

    if prereq_passed == prerequisites[course as usize].len() {
        cleared.insert(course);
        return true;
    } else {
        return false;
    }
}
