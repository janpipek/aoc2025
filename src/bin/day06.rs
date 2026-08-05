use std::fs;
use aoc2025::read_lines;

fn main() {
    println!("Day 6");
    part1();
    part2();
}

struct Input {
    digits: Vec<Vec<Option<u8>>>,
    operators: Vec<Option<char>>,
}

impl Input {
    pub fn len(&self) -> usize {
        self.digits.iter().map(|l| l.len()).max().unwrap()
    }
}

fn is_empty(input: &Input, i: usize) -> bool {
    for v in input.digits.iter() {
        if v[i] != None {
            return false;
        }
    }
    true
}

fn number_at(input: &Input, i: usize) -> u64 {
    let mut n: u64 = 0;
    for v in input.digits.iter() {
        if let Some(number) = v[i] {
            n *= 10;
            n += number as u64;
        }
    }
    n
}

fn find_block_starts(input: &Input) -> Vec<usize> {
    let mut positions: Vec<usize> = vec![];
    for (i, operator) in input.operators.iter().enumerate() {
        if *operator != None {
            positions.push(i);
        }
    }
    positions
}

fn compute_block_at(input: &Input, i: usize) -> u64 {
    let mut numbers: Vec<u64> = vec![];
    let mut j: usize = i;
    while j < input.len() && !is_empty(&input, j) {
        numbers.push(number_at(&input, j));
        j += 1;
    }
    match input.operators[i] {
        None => panic!("Cannot evaluate empty operator"),
        Some('*') => numbers.iter().product(),
        Some('+') => numbers.iter().sum(),
        _ => panic!("Unknown operator")
    }
}

fn parse_input(lines: &Vec<String>) -> Input {
    let columns: usize = lines[0..lines.len() - 1].iter().map(|l| l.len()).max().unwrap();

    let numbers: Vec<Vec<Option<u8>>> = lines[0..lines.len() - 1]
        .iter()
        .map(|l| {
            let mut chars: Vec<Option<u8>> = l.chars().map(|c| c.to_string().parse::<u8>().ok()).collect();
            chars.resize(columns, None);
            chars
        })
        .collect()
        ;
    let operators = lines[lines.len() - 1]
        .chars()
        .map(|c| match c {
            ' ' => None,
            op => Some(op),
        })
        .collect();
    Input { digits: numbers, operators }
}

fn part1() {
    let lines: Vec<String> = read_lines("data/day06.txt");
    let table: Vec<Vec<i64>> = lines[..lines.len() - 1]
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let operators: Vec<String> = lines[lines.len() - 1]
        .split_whitespace()
        .map(|s| String::from(s))
        .collect();
    let result: i64 = (0..table[0].len())
        .map(|i| {
            if operators[i] == "*" {
                table.iter().map(|row| row[i]).product::<i64>() as i64
            } else {
                table.iter().map(|row| row[i]).sum::<i64>()
            }
        })
        .sum();
    println!("Part 1: {result}")
}

fn part2() {
    let lines = fs::read_to_string("data/day06.txt").expect("Cannot read file.").lines().map(|l| String::from(l)).collect();
    let input = parse_input(&lines);

    let mut sum: u64 = 0;
    let block_starts = find_block_starts(&input);
    for i in block_starts {
        sum += compute_block_at(&input, i);
    }
    println!("Part 2: {}", sum);
}
