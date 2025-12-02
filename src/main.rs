use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    day1();
}

fn day1() {
    let mut sum = 50;
    let mut zero_count = 0;

    let file = File::open("data/day01.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.len() == 0 {
            continue;
        }
        let direction = match line.chars().nth(0) {
            Some('R') => 1,
            Some('L') => -1,
            Some(_) => panic!("Invalid direction"),
            None => panic!("No direction")
        };
        let amount: i32 = line[1..].parse().expect("Failed to parse amount");
        sum += direction * amount;
        sum %= 100;
        if sum == 0 {
            zero_count += 1;
        }
    }
    println!("Zero count: {}", zero_count);
}
