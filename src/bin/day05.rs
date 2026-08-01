use aoc2025::read_lines;
use std::cmp::{max, min};

struct Input {
    fresh: Vec<(u64, u64)>,
    available: Vec<u64>,
}

fn read_input() -> Input {
    let lines = read_lines("data/day05.txt");
    let mut fresh: Vec<(u64, u64)> = vec![];
    let mut available: Vec<u64> = vec![];
    lines.iter().for_each(|line| {
        if line.is_empty() {
        } else if line.contains('-') {
            let pair: Vec<u64> = line.split('-').map(|s| s.parse().unwrap()).collect();
            if pair.len() != 2 {
                panic!("A range must contain two numbers");
            }
            fresh.push((pair[0], pair[1]))
        } else {
            let i: u64 = line.parse().unwrap();
            available.push(i);
        }
    });
    Input { fresh, available }
}

fn is_fresh(i: u64, fresh: &Vec<(u64, u64)>) -> bool {
    fresh
        .iter()
        .map(|pair| i >= pair.0 && i <= pair.1)
        .any(|r| r)
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input();
    let count: u64 = input
        .available
        .iter()
        .map(|ingredient| is_fresh(*ingredient, &input.fresh) as u64)
        .sum();
    println!("Part 1: {}", count);
}

fn merge_one_pair(fresh: &Vec<(u64, u64)>) -> (bool, Vec<(u64, u64)>) {
    let mut merged: Vec<(u64, u64)> = vec![];
    for i in 0..(fresh.len() - 1) {
        let first = fresh[i];
        let second = fresh[i + 1];
        if (second.0 > first.1) || (first.0 > second.1) {
            merged.push(first);
            continue;
        };
        let pair = (min(first.0, second.0), max(first.1, second.1));
        merged.push(pair);
        for k in (i + 2)..fresh.len() {
            merged.push(fresh[k]);
        }
        return (true, merged);
    }
    (false, fresh.clone())
}

fn total_size(ranges: &Vec<(u64, u64)>) -> u64 {
    ranges.iter().map(|r| r.1 - r.0 + 1).sum()
}

fn part2() {
    let input = read_input();

    let mut fresh = input.fresh;
    let mut changed = true;

    fresh.sort_by_key(|r| r.0);

    while changed {
        (changed, fresh) = merge_one_pair(&fresh);
    }

    let count = total_size(&fresh);
    println!("Part 2: {}", count);
}
