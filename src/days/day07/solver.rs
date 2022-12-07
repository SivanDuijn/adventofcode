// https://adventofcode.com/2022/day/7

use std::{collections::HashMap};

use advent_of_code_2022::split;

struct Directory {
    parent: Option<usize>,
    dirs: HashMap<String, usize>,
    size: i32,
}

fn parse(input: &str) -> Vec<Directory> {
    let lines = split(input, "\n");

    let mut dirs: Vec<Directory> = Vec::new();

    dirs.push(Directory { parent: None, dirs: HashMap::new(), size: 0 });

    let mut working_dir: usize = 0;

    for line in lines.iter().skip(1) {
        let args = split(line, " ");
        
        if args[0] == "$" {
            if args[1] == "ls" {
                continue;
            }
            else if args[1] == "cd" {
                if args[2] == ".." {
                    let total_child_size = dirs[working_dir].size;
                    working_dir = dirs[working_dir].parent.unwrap();
                    dirs[working_dir].size += total_child_size;
                }
                else {
                    working_dir = dirs[working_dir].dirs[&args[2]];
                }
            }
        }
        else if args[0] == "dir" {
            let new_dir_index = dirs.len();
            dirs.push(Directory { parent: Some(working_dir), dirs: HashMap::new(), size: 0 });
            dirs[working_dir].dirs.insert(args[1].to_owned(), new_dir_index);
        }
        else {
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
    let size = dirs.iter().filter(|&d| d.size <= 100000).map(|d| d.size).sum::<i32>();
    return size.to_string();
}

fn solve2(dirs: &Vec<Directory>) -> String {
    let unused_space = 70000000 - dirs[0].size;

    let mut sorted: Vec<i32> = dirs.iter().map(|d| d.size).collect::<Vec<i32>>();
    sorted.sort();
    return sorted.iter().find(|&v| v + unused_space > 30000000).unwrap().to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);
    return (solve1(&parsed_input), solve2(&parsed_input));
}