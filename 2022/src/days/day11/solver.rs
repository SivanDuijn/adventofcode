// https://adventofcode.com/2022/day/11

use std::collections::VecDeque;

fn parse(input: &str) -> Vec<Monkey> {

    input.split("\n\n").map(|m| 
        m.split("\n").skip(1)
    ).map(|mut m| 
        Monkey {
            items: m.next().unwrap().split(": ").nth(1).unwrap().split(", ").map(|v| v.parse().unwrap()).collect(),
            operation: parse_operator(m.next().unwrap().split("= ").nth(1).unwrap()),
            divisible_by: m.next().unwrap().split("by ").nth(1).unwrap().parse().unwrap(),
            throw_to_when_true: m.next().unwrap().split("monkey ").nth(1).unwrap().parse().unwrap(),
            throw_to_when_false: m.next().unwrap().split("monkey ").nth(1).unwrap().parse().unwrap(),
            n_inspected_items: 0
        }
    ).collect()
}

fn parse_operator(input: &str) -> Operation {
    let words: Vec<&str> = input.split_whitespace().collect();
    Operation {
        rhs: words[2],
        operator: match words[1] {
            "+" => Operator::Add,
            _   => Operator::Multiply,
        }
    }
}

#[derive(PartialEq, Clone)]
enum Operator {
    Add,
    Multiply
}

#[derive(Clone)]
struct Operation<'a> {
    rhs: &'a str,
    operator: Operator
}

impl Operation<'_> {
    pub fn apply_with(&self, old: i64) -> i64 {
        let rhs: i64 = match self.rhs {
            "old" => old,
            _     => self.rhs.parse().unwrap()
        };

        return match self.operator {
            Operator::Add => old + rhs,
            Operator::Multiply => old * rhs
        };
    }
}

#[derive(Clone)]
struct Monkey<'a> {
    items: VecDeque<i64>,
    operation: Operation<'a>,
    divisible_by: i64,
    throw_to_when_true: usize,
    throw_to_when_false: usize,
    n_inspected_items: u32
}

impl Monkey<'_> {
    pub fn inspect_item(&mut self, devide_by: i64, common_devisor: i64) -> (i64, usize) {
        self.n_inspected_items += 1;

        let mut item = self.items.pop_front().unwrap();

        item = self.operation.apply_with(item);
        item /= devide_by;
        item %= common_devisor;
        if item % self.divisible_by == 0 {
            return (item, self.throw_to_when_true);
        }
        else {
            return (item, self.throw_to_when_false);
        }
    }

    pub fn catch_item(&mut self, item: i64) {
        self.items.push_back(item);
    }
}

fn monkeys_go_brrr(monkeys: &mut Vec<Monkey>, rounds: usize, devide_by: i64) -> u64 {
    let common_devisor: i64 = monkeys.iter().map(|m| m.divisible_by).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let (new, throw_to) = monkeys[i].inspect_item(devide_by, common_devisor);
                monkeys[throw_to].catch_item(new);
            }
        }
    }

    let mut sorted: Vec<u32> = monkeys.iter().map(|m| m.n_inspected_items).collect();
    sorted.sort_by(|a, b| b.cmp(a));
    return Into::<u64>::into(sorted[0]) *  Into::<u64>::into(sorted[1]);
}

fn solve1(monkeys: &mut Vec<Monkey>) -> String {
    let monkey_business = monkeys_go_brrr(monkeys, 20, 3);
    return monkey_business.to_string();
}

fn solve2(monkeys: &mut Vec<Monkey>) -> String {
    let monkey_business = monkeys_go_brrr(monkeys, 10000, 1);
    return monkey_business.to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = &mut parse(input);
    return (solve1(&mut parsed_input.clone()), solve2(parsed_input));
}