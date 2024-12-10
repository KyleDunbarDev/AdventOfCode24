use std::fs;
use std::io;

fn main() {
    solve();
    solve_part_2();
}

fn read_grid() -> io::Result<Vec<Vec<char>>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let mut grid = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    Ok(grid)
}

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let grid = read_grid().expect("Failed to read input");
    let word = "XMAS";
    let word_rev: String = word.chars().rev().collect();
    let word_len = word.len();

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            for (dx, dy) in directions {
                //for every given direction, add chars to seq. check seq.
                let mut seq = String::new();
                for k in 0..word_len {
                    let x = i as isize + dx * k as isize;
                    let y = j as isize + dy * k as isize;
                    if x >= 0 && x < rows as isize && y >= 0 && y < cols as isize {
                        seq.push(grid[x as usize][y as usize]);
                    } else {
                        break;
                    }
                }

                if seq.len() == word_len && (seq == word || seq == word_rev) {
                    count += 1;
                }
            }
        }
    }

    println!("Day 4 Part 1: {}", count / 2);

    Ok(())
}

fn check(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if grid[i][j] != 'A' {
        return false;
    }

    return ((grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S')
        || (grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M'))
        && ((grid[i + 1][j - 1] == 'M' && grid[i - 1][j + 1] == 'S')
            || (grid[i + 1][j - 1] == 'S' && grid[i - 1][j + 1] == 'M'));
}

pub fn solve_part_2() -> Result<(), Box<dyn std::error::Error>> {
    let grid = read_grid().expect("Failed to read input");

    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if check(&grid, i, j) {
                count += 1;
            }
        }
    }

    println!("Day 4 Part 2: {}", count);

    Ok(())
}
