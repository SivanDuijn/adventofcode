use std::collections::HashSet;

use advent_of_code::{split, read_file};

pub fn main() {
    let lines = split(read_file("src/bin/day-3/in-1.txt"), "\n");

    let mut total = 0;

    let mut first_rucksack: HashSet<char> = HashSet::new();
    let mut second_rucksack: HashSet<char> = HashSet::new();
    
    let mut rucksack_index = 0;

    while rucksack_index < lines.len() {
        first_rucksack.clear();
        second_rucksack.clear();

        print!("{} ", rucksack_index);

        for i in 0..3 {
            let rucksack = &lines[rucksack_index + i];
            print!("{} ", rucksack);
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
                        print!(" {} {}", v, c);
                        
                        total += v;
                        break;
                    }
                }
                
            }
        }

        println!();

        rucksack_index += 3;
    }

    println!("{total}");

}

pub fn main_first() {
    let lines = split(read_file("src/bin/day-3/in-1.txt"), "\n");

    let mut total = 0;

    let mut i: i32;
    let mut compartment_size: i32;
    let mut first_compartment: HashSet<char> = HashSet::new();
    for rucksack in lines {
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
                    // println!("{} {}", v, c);
                    
                    total += v;
                    break;
                }
            }
            
            i += 1;
        }
    }

    println!("{total}");

}