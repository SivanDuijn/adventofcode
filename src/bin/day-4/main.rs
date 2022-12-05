use advent_of_code::{split, read_file};

pub fn main() {
    let lines = split(read_file("src/bin/day-4/in-1.txt"), "\n");

    let mut pairs: Vec<String>;
    let mut elf_one: Vec<String>;
    let mut elf_two: Vec<String>;
    let mut elf_one_min: i32;
    let mut elf_one_max: i32;
    let mut elf_two_min: i32;
    let mut elf_two_max: i32;

    let mut total = 0;

    for line in lines {
        pairs = split(line, ",");

        elf_one = split((&pairs[0]).to_string(), "-");
        elf_two = split((&pairs[1]).to_string(), "-");

        elf_one_min = elf_one[0].parse().unwrap();
        elf_one_max = elf_one[1].parse().unwrap();
        elf_two_min = elf_two[0].parse().unwrap();
        elf_two_max = elf_two[1].parse().unwrap();

        if     (elf_one_min >= elf_two_min && elf_one_min <= elf_two_max) 
            || (elf_two_min >= elf_one_min && elf_two_min <= elf_one_max) {
            total += 1;
        }
    }

    println!("{total}");
}