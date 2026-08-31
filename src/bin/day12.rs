use std::panic;

use aoc2025::read_lines;
use regex::Regex;

fn main() {
    println!("Day 11");
    let input = read_input("data/day12.txt");
    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    let result2 = part2(&input);
    println!("Part 2: {}", result2);
}

struct Present {
    grid: Vec<Vec<bool>>
}

impl Present {
    fn new() -> Present {
        Present { grid: Vec::new() }
    }
}

struct Region {
    size: (i64, i64),
    presents: Vec<usize>
}

struct Input {
    presents: Vec<Present>,
    regions: Vec<Region>
}

fn read_input(path: &str) -> Input {
    let lines = read_lines(path);
    let mut presents = Vec::new();
    let mut regions = Vec::new();

    let present_regex = Regex::new(r"^([0-9]+):").unwrap();
    let region_regex = Regex::new(r"^([0-9]+)x([0-9])+: (.+)").unwrap();

    let mut current_present = Present{ grid: Vec::new() };

    for line in lines {
        if let Some(_) = present_regex.captures(&line) {
            // We do not need the number
            if current_present.grid.len() > 0 {
                presents.push(current_present);
                current_present = Present::new();
            }
        }
        else if let Some(r) = region_regex.captures(&line) {
            // Reset the presents as well
            if current_present.grid.len() > 0 {
                presents.push(current_present);
                current_present = Present::new();
            }

            let x: i64 = r.get(1).unwrap().as_str().parse().unwrap();
            let y: i64 = r.get(2).unwrap().as_str().parse().unwrap();
            regions.push(
                Region {
                    size: (x, y),
                    presents: r.get(3).unwrap().as_str().split(" ").map(|s| s.parse().unwrap()).collect()
                }
            );
        }
        else {
            current_present.grid.push(
                line.chars().map(|c| c == '#').collect()
            )
        }
    }

    Input {
        presents: presents,
        regions: regions
    }
}

fn part1(input: &Input) -> i64 {
    0
}

fn part2(input: &Input) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = read_input("data/day12-test.txt");
        assert_eq!(input.presents.len(), 6);
        assert_eq!(input.regions.len(), 3);

        assert_eq!(input.presents[5].grid[1], vec![false, true, false]);
        assert_eq!(input.regions[2].size, (12, 5));
    }

    #[test]
    fn test_part1() {
        let input = read_input("data/day12-test.txt");
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = read_input("data/day12-test.txt");
        panic!("Part 2 is not implemented yet.");
    }
}
