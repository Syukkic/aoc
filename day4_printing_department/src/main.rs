use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("No input file");
    let reader = io::BufReader::new(file);
    let input: Vec<String> = reader
        .lines()
        .map(|line| line.expect("failed to read line"))
        .collect();

    // let input = vec![
    //     "..@@.@@@@.",
    //     "@@@.@.@.@@",
    //     "@@@@@.@.@@",
    //     "@.@@@@..@.",
    //     "@@.@@@@.@@",
    //     ".@@@@@@@.@",
    //     ".@.@.@.@@@",
    //     "@.@@@.@@@@",
    //     ".@@@@@@@@.",
    //     "@.@.@@@.@.",
    // ];

    let counter = search_accessibility(input);
    println!("Answer: {}", counter);
}

fn search_accessibility(input: Vec<String>) -> i32 {
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    // grid => [['.', '.', '@', '@', '.', '@', '@', '@', '@', '.'], ['@', '@', '@', '.', '@', '.', '@', '.', '@', '@'], ['@', '@', '@', '@', '@', '.', '@', '.', '@', '@'], ['@', '.', '@', '@', '@', '@', '.', '.', '@', '.'], ['@', '@', '.', '@', '@', '@', '@', '.', '@', '@'], ['.', '@', '@', '@', '@', '@', '@', '@', '.', '@'], ['.', '@', '.', '@', '.', '@', '.', '@', '@', '@'], ['@', '.', '@', '@', '@', '.', '@', '@', '@', '@'], ['.', '@', '@', '@', '@', '@', '@', '@', '@', '.'], ['@', '.', '@', '.', '@', '@', '@', '.', '@', '.']]

    let rows = grid.len();
    let cols = grid[0].len();
    let mut counter = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != '@' {
                continue;
            }

            let mut neighbors = 0;
            let row_start = i.saturating_sub(1);
            let row_end = (i + 1).min(rows - 1);
            let col_start = j.saturating_sub(1);
            let col_end = (j + 1).min(cols - 1);

            (row_start..=row_end).for_each(|ni| {
                (col_start..=col_end).for_each(|nj| {
                    if (ni != i || nj != j) && grid[ni][nj] == '@' {
                        neighbors += 1;
                    }
                });
            });
            if neighbors < 4 {
                counter += 1;
            }
        }
    }
    counter
}
