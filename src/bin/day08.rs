use std::collections::HashSet;
use itertools::Itertools;

use aoc2025::read_lines;

pub fn read_input() -> Vec<(i64, i64, i64)> {
    let lines = read_lines("data/data08.txt");
    lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(",").collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2].parse().unwrap())
    }).collect()
}

fn main() {
    println!("Day 8");
    let input = read_input();
    let result1 = part1(&input, 1000);
    println!("Part 1: {}", result1);
    let result2 = part2(&input);
    println!("Part 2: {}", result2);
}

fn distance_squared(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn maybe_merge_clusters(_connection: &(i64, i64), mut _clusters: &Vec<HashSet<i64>>) -> bool {
    // TODO: Implement
    true
}

fn part1(input: &Vec<(i64, i64, i64)>, steps: i64) -> i64 {
    let mut clusters: Vec<HashSet<i64>> = (0..input.len()).map(|i| HashSet::from([i as i64])).collect();
    let closest_pairs: Vec<(i64, i64)> = (0..input.len()).tuple_combinations().sorted_by(
        |&(a, b), &(c, d)| distance_squared(&input[a], &input[b]).cmp(&distance_squared(&input[c], &input[d]))
    ).collect();

    let mut steps_taken = 0;

    for pair in closest_pairs {
        let (a, b) = pair;
        if maybe_merge_clusters(&pair, &clusters) {
            steps_taken += 1;
            if steps_taken == steps {
                break;
            }
        }
    }
    clusters.iter().map(|cluster| cluster.len()).sorted().rev().take(3).product()
}

fn part2(_input: &Vec<(i64, i64, i64)>) -> i64 {
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
}
