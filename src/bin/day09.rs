use aoc2025::read_lines;

pub fn read_input() -> Vec<(i64, i64)> {
    let lines = read_lines("data/day09.txt");
    lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(",").collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }).collect()
}

fn main() {
    println!("Day 9");
    let input = read_input();
    part1(&input);
    part2(&input);
}

fn rectangle_area(first: (i64, i64), second: (i64, i64)) -> i64 {
    let width = (first.0 - second.0).abs() + 1;
    let height = (first.1 - second.1).abs() + 1;
    width * height
}

fn part1(input: &Vec<(i64, i64)>) {
    let mut max_area = 0;
    for first in input.iter() {
        for second in input.iter() {
            let area = rectangle_area(*first, *second);
            if area > max_area {
                max_area = area;
            }
        }
    }
    println!("Part 1: {}", max_area);
}

fn part2(input: &Vec<(i64, i64)>) {
    println!("Part 2: ---");
}
