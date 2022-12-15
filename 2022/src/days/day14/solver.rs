// https://adventofcode.com/2022/day/14

use std::collections::HashSet;

fn parse(input: &str) -> (HashSet<i32>, i32) {
    let mut rocks: HashSet<i32> = HashSet::new();
    let mut lowest_rock = 0;
    input.split("\n")
        .for_each(|line| 
            line.split(" -> ")
            .map(|coords| 
                coords.split(",").map(|c| c.parse().unwrap()).collect())
            .collect::<Vec<Vec<i32>>>()
            .windows(2)
            .for_each(|points| {
                let a = &points[0];
                let b = &points[1];
                let dx = (b[0] - a[0]).clamp(-1, 1); let dy = (b[1] - a[1]).clamp(-1, 1);
                let mut x = a[0]; let mut y = a[1];
                rocks.insert((y<<16) + x);
                while x != b[0] || y != b[1] {
                    x += dx; y += dy;
                    rocks.insert((y<<16) + x);
                    if y > lowest_rock { lowest_rock = y; }
                }
            }));

    return (rocks, lowest_rock);
}

fn solve1(blocked_paths: &mut HashSet<i32>, lowest_rock: i32) -> String {
    let floor = (lowest_rock+1)<<16;

    let mut counter = 0;
    loop {
        let (v, hit_floor) = may_the_sand_be_falling(blocked_paths, floor);
        if hit_floor { break; }

        blocked_paths.insert(v - (1<<16));
        counter += 1;
    }

    counter.to_string()
}

fn solve2(blocked_paths: &mut HashSet<i32>, lowest_rock: i32) -> String {
    let floor = (lowest_rock+2)<<16;

    let mut counter = 0;
    while !blocked_paths.contains(&500) {
        let (v, _) = may_the_sand_be_falling(blocked_paths, floor);

        blocked_paths.insert(v - 65536);
        counter += 1;
    }

    counter.to_string()
}

/// Returns tuple with the coords where the sand comes to rest and bool if the sand hit the floor of the cave
fn may_the_sand_be_falling(blocked_paths: &HashSet<i32>, floor: i32) -> (i32, bool) {
    let mut v = 66036; // start at 500,1
    loop {
        if v > floor {
            return (v, true);
        }

        if !blocked_paths.contains(&v) {
            // If below is free
            v += 65536;
        }
        else if !blocked_paths.contains(&(v - 1)) {
            // If below left is free
            v += 65535;
        }
        else if !blocked_paths.contains(&(v + 1)) {
            // If below right is free
            v += 65537;
        }
        else {
            return (v, false);
        }
    }
}

pub fn solve(input: &str) -> (String, String) {
    let (rocks, lowest_rock) = parse(input);
    let mut rocks_clone_s1: HashSet<i32> = HashSet::new();
    let mut rocks_clone_s2: HashSet<i32> = HashSet::new();
    rocks.iter().for_each(|v| { rocks_clone_s1.insert(*v); rocks_clone_s2.insert(*v); });

    let solutions = (solve1(&mut rocks_clone_s1, lowest_rock), solve2(&mut rocks_clone_s2, lowest_rock));

    draw_sand(&rocks, &rocks_clone_s1);
    draw_sand(&rocks, &rocks_clone_s2);

    return solutions;
}

fn draw_sand(rocks: &HashSet<i32>, blocked_paths: &HashSet<i32>) {
    let mut out: String = String::new();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    
    blocked_paths.iter().for_each(|&v| {
        let x = v % 65536;
        let y = v / 65536;
        
        if x > max_x { max_x = x; }
        if x < min_x { min_x = x; }
        if y > max_y { max_y = y; }
    });
    min_x = 327;
    max_x = 673;
    println!("{min_x} {max_x}");

    for y in 0..(max_y+2) {
        for x in (min_x-2)..(max_x+2) {
            let v = (y<<16) + x;
            if rocks.contains(&v) { out.push('#'); }
            else if blocked_paths.contains(&v) { out.push('.'); }
            else if v == 500 { out.push('+'); }
            else { out.push(' '); }
        }
        out += "\n";
    }

    println!("{out}");
}