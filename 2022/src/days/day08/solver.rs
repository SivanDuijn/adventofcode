// https://adventofcode.com/2022/day/8

use std::collections::HashSet;

use advent_of_code_2022::split;

fn parse(input: &str) -> Vec<Vec<i32>> {
    let lines: Vec<String> = split(input, "\n");
    let grid: Vec<Vec<i32>> = lines.iter().map(|l| l.chars().map(|c| c as i32 - 0x30).collect()).collect();
    return grid;
}

fn solve1(grid: &Vec<Vec<i32>>) -> String {
    let n_rows: usize = grid.len();
    let n_cols: usize = grid.len();

    let mut visible_tree_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut highest_cols: Vec<i32> = Vec::new();
    for _ in 0..n_cols {
        highest_cols.push(-1);
    }

    for r in 0..n_rows {
        let row = &grid[r];

        let mut highest_in_row = -1;
        for c in 0..n_cols {
            let h = row[c];

            if h > highest_in_row {
                highest_in_row = h;
                visible_tree_locations.insert((r, c));
            }

            if h > highest_cols[c] {
                highest_cols[c] = h;
                visible_tree_locations.insert((r, c));
            }
        }

        highest_in_row = -1;
        let mut c = n_cols - 1;
        loop {
            let h = row[c];

            if h > highest_in_row {
                highest_in_row = h;
                visible_tree_locations.insert((r, c));
            }

            if c == 0 {
                break;
            }
            c -= 1;
        }
    }

    for i in 0..n_cols {
        highest_cols[i] = -1;
    }
    let mut r = n_rows - 1;
    loop {
        let row = &grid[r];

        for c in 0..n_cols {
            let h = row[c];
            if h > highest_cols[c] {
                highest_cols[c] = h;
                visible_tree_locations.insert((r, c));
            }
        }

        if r == 0 {
            break;
        }
        r -= 1;
    }

    return (visible_tree_locations.len()).to_string();
}

fn solve2(grid: &Vec<Vec<i32>>) -> String {
    let n_rows: usize = grid.len();
    let n_cols: usize = grid.len();

    let mut best_view_dinstance = 1;

    for row in 1..(n_rows - 1) {
        for col in 1..(n_cols - 1) {
            let height = grid[row][col];
            let mut view_dinstance = 1;

            // Look left
            let mut trees = 0;
            let mut i = col - 1;
            loop {
                trees += 1;
                if grid[row][i] >= height {
                    break;
                }
                if i == 0 {
                    break;
                }
                i -= 1;
            }
            view_dinstance *= trees;

            // Look right
            trees = 0;
            for i in (col+1)..n_cols {
                trees += 1;
                if grid[row][i] >= height {
                    break;
                }
            }
            view_dinstance *= trees;

            // Look top
            trees = 0;
            i = row - 1;
            loop {
                trees += 1;
                if grid[i][col] >= height {
                    break;
                }
                if i == 0 {
                    break;
                }
                i -= 1;
            }
            view_dinstance *= trees;

            // Look bottom
            trees = 0;
            for i in (row+1)..n_rows {
                trees += 1;
                if grid[i][col] >= height {
                    break;
                }
            }
            view_dinstance *= trees;

            if view_dinstance > best_view_dinstance {
                best_view_dinstance = view_dinstance;
            }
        }
    }

    return best_view_dinstance.to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);
    return (solve1(&parsed_input), solve2(&parsed_input));
}