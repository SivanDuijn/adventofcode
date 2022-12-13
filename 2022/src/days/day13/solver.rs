// https://adventofcode.com/2022/day/13

use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum Packet {
    Value(i32),
    Packets(Vec<Packet>)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Value(v1) => {
                match other {
                    Packet::Value(v2) => v1.cmp(v2),
                    Packet::Packets(_) => Packet::Packets(vec![Packet::Value(*v1)]).cmp(other)
                }
            },
            Packet::Packets(ps_left) => {
                match other {
                    Packet::Value(v2) => self.cmp(&Packet::Packets(vec![Packet::Value(*v2)])),
                    Packet::Packets(ps_right) => {
                        for i in 0..ps_left.len() {
                            if i >= ps_right.len() {
                                return Ordering::Greater;
                            }
    
                            let e = ps_left[i].cmp(&ps_right[i]);
                            if e != Ordering::Equal {
                                return e;
                            }
                        }
                        if ps_left.len() == ps_right.len() {
                            return Ordering::Equal;
                        }
                        return Ordering::Less;
                    }
                }
            }
        }
    }
}

fn parse_packet(tokens: &Vec<String>, start_index: usize) -> (Packet, usize) {
    let mut i = start_index;

    let mut items: Vec<Packet> = Vec::new();

    let mut c = tokens[i].as_str();
    while c != "]" {
        let packet: Packet = match c {
            "[" => { 
                    let (p, new_i) = parse_packet(tokens, i + 1);
                    i = new_i;
                    p
                },
            _   => Packet::Value(c.parse().unwrap())
        };

        items.push(packet);

        i += 1;
        if i >= tokens.len() { break; }

        c = tokens[i].as_str();
    }

    return (Packet::Packets(items), i);
}

fn tokenize(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();

    let mut tokens: Vec<String> = Vec::new();

    let mut i: usize = 0;
    let mut c: char;
    while i < chars.len() {
        c = chars[i];
        match c {
            '[' => tokens.push("[".to_string()),
            ']' => tokens.push("]".to_string()),
            ',' => {},
            _   => {
                let (digits, new_i) = tokenize_digits(&chars, i);
                i = new_i;
                tokens.push(digits);
            }
        }

        i += 1;
    }

    return tokens;
}

fn tokenize_digits(chars: &Vec<char>, start_i: usize) -> (String, usize) {
    let digits: String = chars.iter()
        .skip(start_i)
        .take_while(|&&c| c != ']' && c != ',')
        .collect();
    let new_i = start_i + digits.len() - 1;
    return (digits, new_i);
}

fn parse(input: &str) -> Vec<Packet> {
    input.split("\n")
        .filter(|&l| l != "")
        .map(|packet| parse_packet(&tokenize(packet), 1).0)
        .collect()
}

fn solve1(packets: &Vec<Packet>) -> String {
    let mut correct_indices: Vec<usize> = Vec::new();

    let mut i: usize = 0;
    while i < packets.len() {
        let p1 = &packets[i];
        i += 1;
        let p2 = &packets[i];
        if p1 < p2 {
            correct_indices.push(i/2 + 1);
        }
        i += 1;
    }

    let sum: usize = correct_indices.iter().sum();
    sum.to_string()
}

fn solve2(parsed_input: &Vec<Packet>) -> String {
    let mut packets: Vec<&Packet> = parsed_input.iter().map(|p| p).collect();

    let packet_2 = Packet::Packets(vec![Packet::Packets(vec![Packet::Value(2)])]);
    packets.push(&packet_2);
    let packet_6 = Packet::Packets(vec![Packet::Packets(vec![Packet::Value(6)])]);
    packets.push(&packet_6);

    packets.sort();
    let index_2 = packets.iter().enumerate().find(|&(_, &p)| p == &packet_2).unwrap().0 + 1;
    let index_6 = packets.iter().enumerate().find(|&(_, &p)| p == &packet_6).unwrap().0 + 1;

    (index_2 * index_6).to_string()
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);
    return (solve1(&parsed_input), solve2(&parsed_input));
}