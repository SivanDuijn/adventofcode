// https://adventofcode.com/2022/day/2

use advent_of_code_2022::split;

pub fn solve(input: &str) -> (String, String) {
    let lines = split(input, "\n");

    let mut score = 0;

    for g in lines {
        let shapes = split(&g, " ");
        let opp = &shapes[0];
        let mut me  = shapes[1].as_str();

        if opp == "A" {
            match me {
                "X" => me = "Z",
                "Y" => me = "X",
                _   => me = "Y",
            }
        }
        else if opp == "B" {
            match me {
                "X" => me = "X",
                "Y" => me = "Y",
                _   => me = "Z",
            }
        }
        else if opp == "C" {
            match me {
                "X" => me = "Y",
                "Y" => me = "Z",
                _   => me = "X",
            }
        }

        if me == "X" {
            score += 1;
            if opp == "C" {
                score += 6;
            }
            else if opp == "A" {
                score += 3;
            }
        }
        else if me == "Y" {
            score += 2;
            if opp == "A" {
                score += 6;
            }
            else if opp == "B" {
                score += 3;
            }
        }
        else if me == "Z" {
            score += 3;
            if opp == "B" {
                score += 6;
            }
            else if opp == "C" {
                score += 3;
            }
        }
    }

    return ("".to_string(), score.to_string());
}