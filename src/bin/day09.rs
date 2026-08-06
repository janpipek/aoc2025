use std::cmp::{min, max};
use aoc2025::read_lines;

pub fn read_input(path: &str) -> Vec<(i64, i64)> {
    let lines = read_lines(path);
    lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(",").collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }).collect()
}

fn main() {
    println!("Day 9");
    let input = read_input("data/day09.txt");
    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    let result2 = part2(&input);
    println!("Part 2: {}", result2);
}

fn rectangle_area(first: (i64, i64), second: (i64, i64)) -> i64 {
    let width = (first.0 - second.0).abs() + 1;
    let height = (first.1 - second.1).abs() + 1;
    width * height
}

fn part1(input: &Vec<(i64, i64)>) -> i64 {
    let mut max_area = 0;
    for first in input.iter() {
        for second in input.iter() {
            let area = rectangle_area(*first, *second);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

struct Range {
    min: i64,
    max: i64,
}

struct Range2D {
    x: Range,
    y: Range,
}

fn find_ranges(p: &((i64, i64), (i64, i64))) -> Range2D {
    Range2D {
        x: Range {
            min: min(p.0.0, p.1.0),
            max: max(p.0.0, p.1.0),
        },
        y: Range {
            min: min(p.0.1, p.1.1),
            max: max(p.0.1, p.1.1)
        }
    }
}

fn ranges_cross(r1: &Range, r2: &Range) -> bool {
    if r1.min >= r2.max { false } else if r2.min >= r1.max { false } else { true }
}

fn ranges_cross_2d(r1: &Range2D, r2: &Range2D) -> bool {
    ranges_cross(&r1.x, &r2.x) && ranges_cross(&r1.y, &r2.y)
}

fn intersects(rectangle: ((i64, i64), (i64, i64)), line: ((i64, i64), (i64, i64))) -> bool {
    let rectangle_ranges = find_ranges(&rectangle);
    let line_ranges = find_ranges(&line);
    ranges_cross_2d(&rectangle_ranges, &line_ranges)
}

fn part2(input: &Vec<(i64, i64)>) -> i64 {
    let mut max_area = 0;
    for first in input.iter() {
        for second in input.iter() {
            if first == second { continue; }
            let mut ok: bool = true;
            for i in 0..input.len() {
                let j = (i + 1) % input.len();
                if intersects((*first, *second), (input[i], input[j])) {
                    // Only internal part of the rectangle matters
                    // If there were touching sides, this would fail.
                    // Also, there could be a rectangle outside that would fit
                    // Luckily, neither of this is true in the input data
                    ok = false;
                    break;
                }
            }
            if !ok { continue; }
            let area = rectangle_area(*first, *second);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("data/day09-test.txt");
        let result = part1(&input);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part2() {
        let input = read_input("data/day09-test.txt");
        let result = part2(&input);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_intersects() {
        let rectangle = ((11, 1), (2, 3));
        let line = ((7,1), (7, 3));
        assert!(intersects(rectangle, line));
    }
}