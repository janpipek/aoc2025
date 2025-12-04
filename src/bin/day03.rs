use std::fs;

fn read_lines() -> Vec<String> {
    let lines = fs::read_to_string("data/day03.txt").expect("Cannot read file.");
    lines.lines().map(|l| String::from(l)).filter(|l| l.trim().len() > 0).collect()
}

fn main() {
    println!("Day 3");
    part1();
    part2();
}

fn compute_joltage(line: String) -> u32 {
    let mut digit_position: Vec<(usize, char)> = line.chars().enumerate().collect();
    // Sort by index first so that after reverse, we take the first occurence of each digit
    digit_position.reverse();
    digit_position.sort_by_key(|x| x.1);
    digit_position.reverse();
    let largest = digit_position[0].1.to_digit(10).unwrap();
    let largest_pos = digit_position[0].0;
    if largest_pos == digit_position.len() - 1 {
        // The largest digit is at the end, it will come second
        let other = digit_position[1].1.to_digit(10).unwrap();
        let joltage = 10 * other + largest;
        print!("{} -> {}\n", line, joltage);
        joltage
    } else {
        // The largest digit is not at the end, it will come first
        let other = line[largest_pos + 1..].chars().max().unwrap().to_digit(10).unwrap();
        let joltage = 10 * largest + other;
        print!("{} -> {}\n", line, joltage);
        joltage
    }
}

fn part1() {
    let lines = read_lines();
    let joltages: Vec<u32> = lines.into_iter().map(compute_joltage).collect();
    let total_joltage = joltages.iter().sum::<u32>();
    println!("Part 1: {}", total_joltage);
}

fn part2() {
    println!("Part 2: ---");
}
