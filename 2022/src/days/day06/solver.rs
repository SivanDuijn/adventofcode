// https://adventofcode.com/2022/day/6

use std::collections::VecDeque;

fn get_num_of_chars_after_unique(input: &str, unique_len: &usize) -> usize {
    let mut i = 0;
    let mut v: VecDeque<char> = VecDeque::new();
    for c in input.chars() {
        if v.len() == *unique_len {
            if v.iter().all(|vc| v.iter().filter(|&p| p == vc).count() == 1) {
                break;
            }

            v.pop_front();
        }
        v.push_back(c);

        i += 1;
    }

    return i;
}

pub fn solve(input: &str) -> (String, String) {
    return (
        get_num_of_chars_after_unique(input, &4).to_string(), 
        get_num_of_chars_after_unique(input, &14).to_string());
}