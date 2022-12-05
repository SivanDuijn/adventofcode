use advent_of_code::{split, read_file};

fn main() {
    let lines = split(read_file("src/bin/day-2/1.txt"), "\n");

    let mut score = 0;

    for g in lines {
        let shapes = split(g, " ");
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

    println!("{score}");
}