use std::{collections::VecDeque, fs};

const _INPUT_FILE: &str = "input/lc_0733_flood_fill.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");
    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let mut result: Vec<(Vec<Vec<i32>>, i32, i32, i32)> = vec![];

    let mut i = 0;
    while i + 1 < lines.len() {
        let grid_line = lines[i];
        let values_line = lines[i + 1];

        let matrix = grid_line
            .trim_matches(|c| c == '[' || c == ']')
            .split("],[")
            .map(|row| {
                row.trim_matches(|c| c == '[' || c == ']')
                    .split(',')
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let values: Vec<i32> = values_line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        assert!(values.len() == 3, "Each value line must have 3 integers");
        result.push((matrix, values[0], values[1], values[2]));

        i += 2;
    }

    println!("{:?}", result);

    for line in result {
        println!("{:?}", _flood_fill(line.0, line.1, line.2, line.3));
    }
}

pub fn _flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let mut image = image;

    _set_bfs(&mut image, (sr as usize, sc as usize), color);

    image
}

// (i32, i32) -> (row, col) -> (y, x)
fn _set_bfs(grid: &mut Vec<Vec<i32>>, start: (usize, usize), set: i32) {
    let height = grid.len();
    let width = grid[0].len();
    let target = grid[start.0][start.1];

    let mut fifo: VecDeque<(usize, usize)> = VecDeque::from(vec![start]);
    grid[start.0][start.1] = set;

    while let Some((y, x)) = fifo.pop_front() {
        // up
        if y > 0 && grid[y - 1][x] == target {
            grid[y - 1][x] = set;
            fifo.push_back((y - 1, x));
        }

        // right
        if x < width - 1 && grid[y][x + 1] == target {
            grid[y][x + 1] = set;
            fifo.push_back((y, x + 1));
        }

        // down
        if y < height - 1 && grid[y + 1][x] == target {
            grid[y + 1][x] = set;
            fifo.push_back((y + 1, x));
        }

        // left
        if x > 0 && grid[y][x - 1] == target {
            grid[y][x - 1] = set;
            fifo.push_back((y, x - 1));
        }
    }
}
