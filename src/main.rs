mod days;

use std::env;
use advent_of_code_2022::read_file;
use days::*;

pub fn main() {
    let day:usize = env::args().nth(1).unwrap_or_default().parse().unwrap_or_default();

    if !(1..=SOLVERS.len()).contains(&day) {
        println!("Please provide a valid day as argument");
        return;
    }

    let example_input = read_file(format!("src/days/day{:02}/example_input.txt", day).as_str());
    let input = read_file(format!("src/days/day{:02}/input.txt", day).as_str());

    let solver = SOLVERS[day-1];

    let (example_solution1, example_2) = solver(&example_input);
    println!("\nexample solutions: {example_solution1} {example_2}");

    // If the argument e is provided, only solve for the example input
    if env::args().nth(2).unwrap_or_default() != "e" {
        let (solution1, solutions2) = solver(&input);
        println!(  "solutions:         {solution1} {solutions2}");
    } 

}

const SOLVERS: [&dyn Fn(&str) -> (String, String); 6] = [
    &day01::solver::solve,
    &day02::solver::solve,
    &day03::solver::solve,
    &day04::solver::solve,
    &day05::solver::solve,
    &day06::solver::solve,
];