mod days;

use std::env;
use advent_of_code_2022::read_file;
use days::*;

use crate::days::solutions::*;

pub fn main() {
    let day:usize = env::args().nth(1).unwrap_or_default().parse().unwrap_or_default();

    if !(1..=SOLVERS.len()).contains(&day) {
        println!("Please provide a valid day as argument");
        return;
    }

    let example_input = read_file(format!("src/days/day{:02}/example_input.txt", day).as_str());
    let input = read_file(format!("src/days/day{:02}/input.txt", day).as_str());

    let solver = SOLVERS[day-1];

    println!();
    format_output("example: ", &solver(&example_input), &EXAMPLE_SOLUTIONS[day-1]);

    // If the argument e is provided, only solve for the example input
    if env::args().nth(2).unwrap_or_default() != "e" {
        format_output("solution:", &solver(&input), &SOLUTIONS[day-1]);
    }

}

fn format_output(prefix: &str, solution: &(String, String), truth: &(&str, &str)) {
    println!("{prefix} {}{}  {}{}", 
        if solution.0 == truth.0 {"🎉 "} else {"❗️ "},
        solution.0, 
        if solution.1 == truth.1 {"🎉 "} else {"❗️ "},
        solution.1);
}

const SOLVERS: [&dyn Fn(&str) -> (String, String); 11] = [
    &day01::solver::solve,
    &day02::solver::solve,
    &day03::solver::solve,
    &day04::solver::solve,
    &day05::solver::solve,
    &day06::solver::solve,
    &day07::solver::solve,
    &day08::solver::solve,
    &day09::solver::solve,
    &day10::solver::solve,
    &day11::solver::solve,
];