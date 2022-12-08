// https://adventofcode.com/2022/day/7

use advent_of_code_2022::split;

struct Directory {
    parent: Option<usize>,
    size: i32,
}

fn parse(input: &str) -> Vec<Directory> {
    let lines = input.split("\n");

    let mut dirs: Vec<Directory> = Vec::new();
    dirs.push(Directory { parent: None, size: 0 });

    let mut working_dir: usize = 0;

    for line in lines.skip(1) {
        let args = split(line, " ");
        
        if args[0] == "$" {
            if args[1] == "cd" {
                if args[2] == ".." {
                    let total_child_size = dirs[working_dir].size;
                    working_dir = dirs[working_dir].parent.unwrap();
                    dirs[working_dir].size += total_child_size;
                }
                else {
                    dirs.push(Directory { parent: Some(working_dir),  size: 0 });
                    working_dir = dirs.len() - 1;
                }
            }
        }
        else if args[0] != "dir" {
            dirs[working_dir].size += args[0].parse::<i32>().unwrap();
        }
    }

    // make to sure to go back to the root to collect all sizes
    while working_dir != 0 {
        let total_child_size = dirs[working_dir].size;
        working_dir = dirs[working_dir].parent.unwrap();
        dirs[working_dir].size += total_child_size;
    }

    return dirs;
}

fn solve1(dirs: &Vec<Directory>) -> String {
    return dirs.iter()
        .filter(|&d| d.size <= 100000)
        .map(|d| d.size)
        .sum::<i32>()
        .to_string();
}

fn solve2(dirs: &Vec<Directory>) -> String {
    let space_needed = 30000000 - (70000000 - dirs[0].size);

    return dirs.iter()
        .map(|d| d.size)
        .filter(|&s| s >= space_needed)
        .min()
        .unwrap()
        .to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);
    return (solve1(&parsed_input), solve2(&parsed_input));
}