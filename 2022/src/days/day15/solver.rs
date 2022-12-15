// https://adventofcode.com/2022/day/15

struct Sensor {
    x: i32,
    y: i32,
    dist_to_closest_beacon: i32
}

fn parse(input: &str) -> usize {
    input.split("\n").map(|line| {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let coords: Vec<Vec<Vec<&str>>> = line
            .split(":")
            .map(|s| 
                s.split(",").map(|s| 
                    s.split("=").collect()
                ).collect()
            ).collect();

        
    });

    1
}

fn solve1(parsed_input: &usize) -> String {
    "?".to_string()
}

fn solve2(parsed_input: &usize) -> String {
    "?".to_string()
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);
    return (solve1(&parsed_input), solve2(&parsed_input));
}