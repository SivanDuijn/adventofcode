// https://adventofcode.com/2022/day/12

use std::collections::VecDeque;

struct Area {
    height: i32,
    neighbors: Vec<usize>
}

fn parse(input: &str) -> Vec<Area> {
    let lines: Vec<Vec<char>> = input.split("\n").map(|l| l.chars().collect()).collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let n_areas = rows*cols;
    
    let mut heightmap: Vec<Area> = Vec::new();
    lines.iter().enumerate().for_each(|(y, row)| 
            row.iter().enumerate().for_each(|(x, c)| {
                let i = y*cols+x;
                let mut neighbors: Vec<usize> = Vec::new(); 
                if i > 0              { neighbors.push(i - 1); }
                if i + 1 < n_areas    { neighbors.push(i + 1); }
                if i >= cols          { neighbors.push(i - cols); }
                if i + cols < n_areas { neighbors.push(i + cols); }
                
                if *c == 'S' {
                    heightmap.push(Area { height: 48, neighbors });
                }
                else if *c == 'E' {
                    heightmap.push(Area { height: 75, neighbors });
                }
                else {
                    heightmap.push(Area { height: *c as i32 - 0x30, neighbors });
                }
            })
        );

    return heightmap;
}

fn plan_route(map: &Vec<Area>, start: usize) -> Option<i32> {
    let mut visited: Vec<bool> = vec![false; map.len()];
    let mut parents: Vec<usize> = vec![0; map.len()];

    let mut q: VecDeque<usize> = VecDeque::new();
    visited[start] = true;
    q.push_back(start);

    while !q.is_empty() {
        let mut area_i = q.pop_front().unwrap();
        let area = &map[area_i];

        // If goal is reached, break
        if area.height == 75 {
            let mut steps = 0;
            while area_i != start {
                area_i = parents[area_i];
                steps += 1;
            }
            return Some(steps);
        }

        for n_i in 0..area.neighbors.len() {
            let n = area.neighbors[n_i];
            if !visited[n] && map[n].height - area.height <= 1 {
                visited[n] = true;
                parents[n] = area_i;
                q.push_back(n);
            }
        }
    }

    return None;    
}

fn solve1(map: &Vec<Area>) -> String {
    // Find start area
    let start: usize = map.iter().position(|a| a.height == 48).unwrap();

    return plan_route(map, start).unwrap().to_string();
}

fn solve2(map: &Vec<Area>) -> String {
    // Find start areas
    let starts: Vec<usize> = map.iter()
        .enumerate()
        .filter(|(_, a)| a.height == 48 || a.height == 49)
        .map(|(i,_)| i)
        .collect();

    let best_path_length: i32 = starts.iter()
        .filter_map(|&s| plan_route(map, s))
        .min()
        .unwrap();

    best_path_length.to_string()
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = &mut parse(input);
    return (solve1(parsed_input), solve2(&parsed_input));
}