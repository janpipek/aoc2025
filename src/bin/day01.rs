use std::{fs::File, io::{BufRead, BufReader}};

fn parse_line(line: &String) -> i32 {
    if line.len() == 0 {
        0
    } else {
        let direction = match line.chars().nth(0) {
            Some('R') => 1,
            Some('L') => -1,
            Some(_) => panic!("Invalid direction"),
            None => panic!("No direction")
        };
        let amount: i32 = line[1..].parse().expect("Failed to parse amount");
        direction * amount
    }
}

fn read_movements() -> Vec<i32> {
    let file = File::open("data/day01.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut movements = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        movements.push(parse_line(&line));
    }
    movements
}

fn main() {
    println!("Day 1");
    part1();
    part2();
}

fn part1() {
    let mut current = 50;
    let mut zeros = 0;
    for movement in read_movements() {
        current = (current + movement) % 100;
        if current == 0 {
            zeros += 1;
        }
    }
    println!("Part 1: zero count = {}", zeros);
}

fn part2() {
    let mut current = 50;
    let mut zeros = 0;
    let movements = read_movements();
    for movement in movements {
        for _ in 0..movement.abs() {
            // No need to be clever here :-D
            current = (current + movement.signum()) % 100;
            if current == 0 {
                zeros += 1;
            }
        }
    }
    println!("Part 2: zero count = {}", zeros);
}
