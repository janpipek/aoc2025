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

fn compute_joltage_1(line: String) -> u32 {
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
        joltage
    } else {
        // The largest digit is not at the end, it will come first
        let other = line[largest_pos + 1..].chars().max().unwrap().to_digit(10).unwrap();
        let joltage = 10 * largest + other;
        joltage
    }
}

fn parse_batteries(line: String) -> Vec<u8> {
    line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn maximise_batteries(batteries: &Vec<u8>, select_count: i8) -> Vec<u8> {
    let batteries_count = batteries.len() as i8;
    if select_count > batteries_count {
        panic!("Cannot select more batteries than provided.");
    }
    if select_count == batteries_count {
        return batteries.clone();
    }
    let candidates = batteries.iter().take((batteries_count - select_count) as usize + 1);
    let (pos, maximum) = candidates.enumerate().reduce(|best, current| {
        if current.1 > best.1 {
            current
        } else {
            best
        }
    }).unwrap();
    let mut result = vec![*maximum];
    if select_count > 1 {
        let remaining: Vec<u8> = batteries.iter().skip(pos as usize + 1).map(|x| *x).collect();
        result.extend(
            maximise_batteries(&remaining, select_count - 1)
        );
    }
    result
}

fn compute_joltage(batteries: Vec<u8>) -> u64 {
    let mut result: u64 = 0;
    for b in  batteries.iter() {
        result *= 10;
        result += *b as u64;
    }
    result
}

fn part1() {
    let lines = read_lines();
    let joltages: Vec<u32> = lines.into_iter().map(compute_joltage_1).collect();
    let total_joltage = joltages.iter().sum::<u32>();
    println!("Part 1: {}", total_joltage);
}

fn part2() {
    println!("Part 2: ---");
    let lines = read_lines();
    let joltages = lines.into_iter().map(parse_batteries).map(|b| maximise_batteries(&b, 12)).map(compute_joltage);
    let total_joltage: u64 = joltages.sum();
    println!("Part 2: {}", total_joltage);
}
