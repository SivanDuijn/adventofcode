// https://adventofcode.com/2022/day/3

use std::collections::HashSet;

use advent_of_code_2022::split;

fn parse(input: &str) -> Vec<String> { return split(input, "\n"); }

fn solve1(parsed_input: &Vec<String>) -> String {
    let mut total = 0;

    let mut i: i32;
    let mut compartment_size: i32;
    let mut first_compartment: HashSet<char> = HashSet::new();
    for rucksack in parsed_input {
        first_compartment.clear();
        i = 0;
        compartment_size = rucksack.len().try_into().unwrap();
        compartment_size >>= 1;
        for c in rucksack.chars() {
            if i < compartment_size {
                first_compartment.insert(c);
            }
            else {
                if first_compartment.contains(&c) {
                    let mut v = c as u32;
                    if v > 97 {
                        v -= 96;
                    }
                    else {
                        v -= 38;
                    }
                    
                    total += v;
                    break;
                }
            }
            
            i += 1;
        }
    }

    return total.to_string();
}

fn solve2(parsed_input: &Vec<String>) -> String { 
    let mut total = 0;

    let mut first_rucksack: HashSet<char> = HashSet::new();
    let mut second_rucksack: HashSet<char> = HashSet::new();
    
    let mut rucksack_index = 0;

    while rucksack_index < parsed_input.len() {
        first_rucksack.clear();
        second_rucksack.clear();

        for i in 0..3 {
            let rucksack = &parsed_input[rucksack_index + i];
            for c in rucksack.chars() {
                if i == 0 {
                    first_rucksack.insert(c);
                }
                else if i == 1 {
                    second_rucksack.insert(c);
                }
                else {
                    if first_rucksack.contains(&c) && second_rucksack.contains(&c) {
                        let mut v = c as u32;
                        if v > 97 {
                            v -= 96;
                        }
                        else {
                            v -= 38;
                        }
                        
                        total += v;
                        break;
                    }
                }
                
            }
        }

        rucksack_index += 3;
    }

    return total.to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);
    return (solve1(&parsed_input), solve2(&parsed_input));
}