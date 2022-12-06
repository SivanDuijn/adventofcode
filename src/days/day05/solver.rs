// https://adventofcode.com/2022/day/5

use advent_of_code_2022::split;

fn parse(input: &str) -> Vec<String> { return split(input, "\n"); }


fn solve2(parsed_input: &Vec<String>) -> String { 
    let mut i: usize; 
    let mut parsing_moves: bool = false;

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut reversed_stacks: Vec<Vec<char>> = Vec::new();

    for line in parsed_input {
        if line == "" {
            parsing_moves = true;
            for i in 0..stacks.len() {
                reversed_stacks.push(Vec::new());
                while !stacks[i].is_empty() {
                    let c = stacks[i].pop().unwrap();
                    reversed_stacks[i].push(c);
                    // print!("{}", c)
                }
                // println!();
            }
            continue;
        }
        // print!("{line}  ");
        
        i = 0;
        if !parsing_moves {
            for c in line.chars().skip(1).step_by(4) {
                if c == '1' {
                    break;
                }
                if c == ' ' {
                    i += 1;
                    continue;
                }
                while stacks.len() <= i {
                    stacks.push(Vec::new());
                }
                stacks[i].push(c);

                i += 1;
            }
        } 
        else {
            let words = split(&line, " ");
            let amount: u32 = words[1].parse().unwrap();
            let from: u32 = words[3].parse().unwrap();
            let to: u32 = words[5].parse().unwrap();

            // print!("{} {} {} ", amount, from, to);

            let mut stack: Vec<char> = Vec::new();
            for _ in 0..amount {
                let c = reversed_stacks[usize::try_from(from).unwrap() - 1].pop().unwrap();
                stack.push(c);
            }

            while !stack.is_empty() {
                let c = stack.pop().unwrap();
                reversed_stacks[usize::try_from(to).unwrap() - 1].push(c);
            }
            
            // for i in 0..reversed_stacks.len() {
            //     if reversed_stacks[i].is_empty() {
            //         continue;
            //     }
            //     let c = reversed_stacks[i].last().unwrap();
            //     print!("{c}")
            // }
        }

        // println!();
    }

    let mut solution = String::new();
    for mut stack in reversed_stacks {
        solution.push(stack.pop().unwrap());
    }

    // println!();
    // for mut stack in reversed_stacks {
    //     let c = stack.pop().unwrap();
    //     print!("{c}");
    // }
    // println!(); }

    return solution;
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);

    return ("".to_string(), solve2(&parsed_input));
}