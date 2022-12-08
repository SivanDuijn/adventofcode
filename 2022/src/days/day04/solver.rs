// https://adventofcode.com/2022/day/4

use advent_of_code_2022::split;

fn parse(input: &str) -> Vec<String> { return split(input, "\n"); }

fn solve1(parsed_input: &Vec<String>) -> String {
    let mut pairs: Vec<String>;
    let mut elf_one: Vec<String>;
    let mut elf_two: Vec<String>;
    let mut elf_one_min: i32;
    let mut elf_one_max: i32;
    let mut elf_two_min: i32;
    let mut elf_two_max: i32;

    let mut total = 0;

    for line in parsed_input {
        pairs = split(line, ",");

        elf_one = split(&pairs[0], "-");
        elf_two = split(&pairs[1], "-");

        elf_one_min = elf_one[0].parse().unwrap();
        elf_one_max = elf_one[1].parse().unwrap();
        elf_two_min = elf_two[0].parse().unwrap();
        elf_two_max = elf_two[1].parse().unwrap();

        if     (elf_one_min >= elf_two_min && elf_one_max <= elf_two_max) 
            || (elf_two_min >= elf_one_min && elf_two_max <= elf_one_max) {
            total += 1;
        }
    }

    return total.to_string();
}

fn solve2(parsed_input: &Vec<String>) -> String { 
    let mut pairs: Vec<String>;
    let mut elf_one: Vec<String>;
    let mut elf_two: Vec<String>;
    let mut elf_one_min: i32;
    let mut elf_one_max: i32;
    let mut elf_two_min: i32;
    let mut elf_two_max: i32;

    let mut total = 0;

    for line in parsed_input {
        pairs = split(line, ",");

        elf_one = split(&pairs[0], "-");
        elf_two = split(&pairs[1], "-");

        elf_one_min = elf_one[0].parse().unwrap();
        elf_one_max = elf_one[1].parse().unwrap();
        elf_two_min = elf_two[0].parse().unwrap();
        elf_two_max = elf_two[1].parse().unwrap();

        if     (elf_one_min >= elf_two_min && elf_one_min <= elf_two_max) 
            || (elf_two_min >= elf_one_min && elf_two_min <= elf_one_max) {
            total += 1;
        }
    }

    return total.to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);
    return (solve1(&parsed_input), solve2(&parsed_input));
}