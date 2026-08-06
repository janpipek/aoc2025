use std::collections::HashSet;
use aoc2025::read_lines;

struct Input {
    width: usize,
    height: usize,
    start: usize,
    is_splitter: Vec<Vec<bool>>
}

fn parse_input(lines: &Vec<String>) -> Input {
    Input {
        height: lines.len(),
        width: lines[0].len(),
        start: lines[0].find('S').unwrap(),
        is_splitter: lines[1..lines.len()].iter().map(
            |l| l.chars().map(|c| c == '^').collect()
        ).collect()
    }
}

fn read_input(path: &str) -> Input {
    let lines = read_lines(path);
    parse_input(&lines)
}

fn main() {
    println!("Day 7");
    let input = read_input("data/day07.txt");
    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    let result2 = part2(&input);
    println!("Part 2: {}", result2);
}

fn part1(input: &Input) -> u64 {
    let mut splits: u64 = 0;
    let mut beams = HashSet::new();
    beams.insert(input.start);
    for i in 1..input.height {
        let splitters: &Vec<bool> = &input.is_splitter[i-1];
        let mut new_beams = HashSet::new();
        for j in beams.iter() {
            if splitters[*j] {
                new_beams.insert(j-1);
                new_beams.insert(j+1);
                splits += 1;
            } else {
                new_beams.insert(*j);
            }
        }
        beams = new_beams;
    }
    splits
}

fn part2(input: &Input) -> u64 {
    let mut histories: Vec<u64> = vec![0; input.width];
    histories[input.start] = 1;
    for i in 1..input.height {
        let mut new_histories = vec![0; input.width];
        let splitters: &Vec<bool> = &input.is_splitter[i-1];
        for (j, count) in histories.iter().enumerate() {
            if splitters[j] {
                new_histories[j-1] += count;
                new_histories[j+1] += count;
            } else {
                new_histories[j] += count;
            }
        }
        histories = new_histories;
    }
    histories.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("data/day07-test.txt");
        let result = part1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2() {
        let input = read_input("data/day07-test.txt");
        let result = part2(&input);
        assert_eq!(result, 40);
    }
}
