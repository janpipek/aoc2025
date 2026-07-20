use std::collections::HashSet;
use itertools::Itertools;

use aoc2025::read_lines;

pub fn parse_input(lines: &Vec<String) -> (uint, Vec<Vec<uint>>) {
    let lines_iter = lines.iter();
    let first_line = lines_iter.next().unwrap();

    lines.iter().map(|line| {
        line.chars().enumerate().map(|(i, c)| {
            if c == ' ' {
                i
            } else {
                i + 1
            }
        }).collect()
    }).collect()
}

pub fn read_input() -> Vec<(usize, usize, usize)> {
    let lines = read_lines("data/day08.txt");
    lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(",").collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2].parse().unwrap())
    }).collect()
}

fn main() {
    println!("Day 7");
    let input = read_input();
    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    let result2 = part2(&input);
    println!("Part 2: {}", result2);
}

fn part1(input: &Vec<(usize, usize, usize)>) -> usize {
    0
}

fn part2(input: &Vec<(usize, usize, usize)>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            (162,817,812),
            (57,618,57),
            (906,360,560),
            (592,479,940),
            (352,342,300),
            (466,668,158),
            (542,29,236),
            (431,825,988),
            (739,650,466),
            (52,470,668),
            (216,146,977),
            (819,987,18),
            (117,168,530),
            (805,96,715),
            (346,949,466),
            (970,615,88),
            (941,993,340),
            (862,61,35),
            (984,92,344),
            (425,690,689)
        ];

        let result = part1(&input, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            (162,817,812),
            (57,618,57),
            (906,360,560),
            (592,479,940),
            (352,342,300),
            (466,668,158),
            (542,29,236),
            (431,825,988),
            (739,650,466),
            (52,470,668),
            (216,146,977),
            (819,987,18),
            (117,168,530),
            (805,96,715),
            (346,949,466),
            (970,615,88),
            (941,993,340),
            (862,61,35),
            (984,92,344),
            (425,690,689)
        ];

        let result = part2(&input);
        assert_eq!(result, 25272);
    }
}
