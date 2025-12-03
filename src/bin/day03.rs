use std::fs;

fn read_lines() -> Vec<String> {
    let lines = fs::read_to_string("data/day03.txt").expect("Cannot read file.");
    lines.lines().map(|l| String::from(l)).collect()
}

fn main() {
    println!("Day 3");
    part1();
    part2();
}

fn compute_joltage(line: String) -> i32 {
    1
}

fn part1() {
    let lines = read_lines();
    let _ = lines.into_iter().map(compute_joltage);
}

fn part2() {
    println!("Part 2: ---");
}
