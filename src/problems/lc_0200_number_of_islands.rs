use std::collections::VecDeque;
use std::fs;

const _INPUT_FILE: &str = "input/lc_0200_number_of_islands.txt";

pub fn _run() {
    let input = fs::read_to_string(_INPUT_FILE).expect("Input file not found");

    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let mut result: Vec<Vec<Vec<char>>> = vec![];

    for line in lines {
        let line = line.trim_matches(|c| c == '[' || c == ']');
        let mut matrix: Vec<Vec<char>> = vec![];
        for row in line
            .split("],[")
            .map(|r| r.trim_matches(|c| c == '[' || c == ']'))
        {
            let row_vec = row
                .split(',')
                .map(|s| s.trim().trim_matches('"').chars().next().unwrap())
                .collect::<Vec<char>>();
            matrix.push(row_vec);
        }

        result.push(matrix);
    }

    println!("{:?}", result);

    for line in result {
        println!("{:?}", _num_islands(line));
    }
}

pub fn _num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid: Vec<Vec<i32>> = _convert_char_matrix_to_i32(grid);

    let mut num_islands = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 1 {
                num_islands += 1;
                _set_bfs(&mut grid, (y, x), -num_islands);
            }
        }
    }

    num_islands
}

// (i32, i32) -> (row, col) -> (y, x)
fn _set_bfs(grid: &mut Vec<Vec<i32>>, start: (usize, usize), set: i32) {
    let height = grid.len();
    let width = grid[0].len();

    if grid[start.0][start.1] != 1 {
        return;
    }

    let mut fifo: VecDeque<(usize, usize)> = VecDeque::from(vec![start]);
    grid[start.0][start.1] = set;

    while let Some((y, x)) = fifo.pop_front() {
        // up
        if y > 0 && grid[y - 1][x] == 1 {
            grid[y - 1][x] = set;
            fifo.push_back((y - 1, x));
        }

        // right
        if x < width - 1 && grid[y][x + 1] == 1 {
            grid[y][x + 1] = set;
            fifo.push_back((y, x + 1));
        }

        // down
        if y < height - 1 && grid[y + 1][x] == 1 {
            grid[y + 1][x] = set;
            fifo.push_back((y + 1, x));
        }

        // left
        if x > 0 && grid[y][x - 1] == 1 {
            grid[y][x - 1] = set;
            fifo.push_back((y, x - 1));
        }
    }
}

fn _convert_char_matrix_to_i32(matrix: Vec<Vec<char>>) -> Vec<Vec<i32>> {
    matrix
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}
