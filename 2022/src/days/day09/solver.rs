// https://adventofcode.com/2022/day/9

use std::{ops::{Sub, AddAssign}, collections::HashSet};

fn parse(input: &str) -> Vec<(&str, i32)> {
    return input.split("\n")
        .map(|l| l.split(" "))
        .map(|mut v| (v.nth(0).unwrap(), v.nth(0).unwrap().parse::<i32>().unwrap()))
        .collect::<Vec<(&str, i32)>>();
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Clamps the values of x and y to -1 and 1
    pub fn clamp_to_one(&mut self) {
        self.x = self.x.clamp(-1, 1);
        self.y = self.y.clamp(-1, 1);
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

fn simulate_moves(motions: &Vec<(&str, i32)>, n_knots: usize) -> String {
    let mut knots: Vec<Point> = Vec::new();
    for _ in 0..n_knots {
        knots.push(Point::zero());
    }

    let mut points_tail_visited:HashSet<Point> = HashSet::new();
    points_tail_visited.insert(Point::zero());

    let movement = &mut Point::zero();
    for &(dir, mut amount) in motions {
        match dir {
            "R" => movement.x =  1,
            "L" => movement.x = -1,
            "U" => movement.y =  1,
            "D" => movement.y = -1,
            _ => {}
        }

        while amount > 0 {
            knots[0] += *movement;

            for i in 0..(n_knots - 1) {
                let mut diff = knots[i] - knots[i + 1];
                
                if diff.x*diff.x + diff.y*diff.y >= 4 {
                    // We need to move this knot
                    diff.clamp_to_one();
                    knots[i + 1] += diff;

                    if i == n_knots - 2 {
                        // If the last knot (the tail) moved, insert it into our hashmap
                        points_tail_visited.insert(knots[n_knots - 1]);
                    }
                }
            }
            amount -= 1;
        }

        movement.x = 0;
        movement.y = 0;
    }

    return points_tail_visited.len().to_string();
}

fn solve1(parsed_input: &Vec<(&str, i32)>) -> String { return simulate_moves(parsed_input, 2); }

fn solve2(parsed_input: &Vec<(&str, i32)>) -> String { return simulate_moves(parsed_input, 10); }

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);
    return (solve1(&parsed_input), solve2(&parsed_input));
}