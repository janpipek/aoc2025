use std::fs;

fn main() {
    println!("Day 2");
    part1();
    part2();
}

fn is_valid(number: i64) -> bool {
    let mut s = number.to_string();
    if s.len() % 2 > 0 {
        return true
    }
    let half_len = s.len() / 2;
    let t = s.split_off(half_len);
    s != t
}

fn is_valid_part2(number: i64) -> bool {
    let n_digits = number.to_string().len();
    for n_repeats in 2..=n_digits {
        if n_digits % n_repeats > 0 {
            continue;
        }
        let block_digits = n_digits / n_repeats;
        let block = 10_i64.pow(block_digits as u32);
        let mut divisor: i64 = 1;
        for _ in 1..n_repeats {
            divisor = divisor * block + 1;
        }
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

fn sum_invalid(lower: i64, upper: i64) -> i64 {
    let mut sum = 0;
    for number in lower..=upper {
        if !is_valid(number) {
            sum += number;
        }
    }
    sum
}

fn sum_invalid_part2(lower: i64, upper: i64) -> i64 {
    let mut sum = 0;
    for number in lower..=upper {
        if !is_valid_part2(number) {
            sum += number;
        }
    }
    sum
}

fn parse_ranges() -> Vec<(i64, i64)> {
    let input = fs::read_to_string("data/day02.txt").expect("Cannot read file");
    let range_strings = input.lines().next().unwrap().split(",");
    let ranges = range_strings.map(|range_str| {
        let parts: Vec<&str> = range_str.trim().split('-').collect();
        if parts.len() != 2 {
            panic!("Wrong size of parts '{}' -> {}", range_str, parts.len());
        }
        let lower: i64 = parts[0].parse().unwrap();
        let upper: i64 = parts[1].parse().unwrap();
        (lower, upper)
    });
    ranges.collect()
}

fn part1() {
    let ranges = parse_ranges();
    let mut sum: i64 = 0;
    for (lower, upper) in ranges {
        sum += sum_invalid(lower, upper)
    }
    println!("Part 1: {}", sum);
}

fn part2() {
    let ranges = parse_ranges();
    let mut sum: i64 = 0;
    for (lower, upper) in ranges {
        sum += sum_invalid_part2(lower, upper)
    }
    println!("Part 2: {}", sum);
}
