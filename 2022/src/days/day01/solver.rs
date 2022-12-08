// https://adventofcode.com/2022/day/1

use advent_of_code_2022::{split_lines, split};


pub fn solve(input: &str) -> (String, String) {
    let elves = split_lines(&split(input, "\n\n"), "\n");

    let calories: Vec<Vec<i32>> = elves.iter().map(|e| e.iter().map(|c| c.parse::<i32>().unwrap()).collect()).collect();

    let summed_calories: Vec<i32> = calories.iter().map(|e| e.iter().sum()).collect();
    
    let mut max_calories1: i32 = 0;
    for s in summed_calories.iter() {
        if s.to_owned() > max_calories1 {
            max_calories1 = *s;
        }
    }

    let mut max_calories2: i32 = 0;
    for s in summed_calories.iter() {
        if s.to_owned() > max_calories2 && s.to_owned() != max_calories1 {
            max_calories2 = *s;
        }
    }

    let mut max_calories3: i32 = 0;
    for s in summed_calories.iter() {
        if s.to_owned() > max_calories3 && s.to_owned() != max_calories1 && s.to_owned() != max_calories2 {
            max_calories3 = *s;
        }
    }

    return (max_calories1.to_string(), (max_calories1 + max_calories2 + max_calories3).to_string())
}