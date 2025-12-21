use std::collections::HashSet;
use itertools::Itertools;

use aoc2025::read_lines;

pub fn read_input() -> Vec<(usize, usize, usize)> {
    let lines = read_lines("data/day08.txt");
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

fn distance_squared(a: &(usize, usize, usize), b: &(usize, usize, usize)) -> usize {
    ((a.0 as i64 - b.0 as i64).pow(2) + (a.1 as i64 - b.1 as i64).pow(2) + (a.2 as i64 - b.2 as i64).pow(2)) as usize
}

fn maybe_merge_clusters(_connection: &(usize, usize), clusters: &mut Vec<HashSet<usize>>) {
    for (i1, cluster1) in clusters.clone().iter().enumerate() {
        if cluster1.contains(&_connection.0) {
            if cluster1.contains(&_connection.1) {
                return;
            }
            for (i2, cluster2) in clusters.clone().iter().enumerate() {
                if i1 == i2 {
                    continue;
                }
                if cluster2.contains(&_connection.1) {
                    let merged_cluster = cluster1.union(cluster2).cloned().collect();
                    clusters[i1] = merged_cluster;
                    clusters.remove(i2);
                    return;
                }
            }
        }
    }
    panic!("We should not have reached this point.")
}

fn part1(input: &Vec<(usize, usize, usize)>, max_steps: usize) -> usize {
    let mut clusters: Vec<HashSet<usize>> = (0..input.len()).map(|i| HashSet::from([i as usize])).collect();
    let closest_pairs: Vec<(usize, usize)> = (0..input.len()).tuple_combinations().sorted_by(
        |&(a, b), &(c, d)| distance_squared(&input[a], &input[b]).cmp(&distance_squared(&input[c], &input[d]))
    ).collect();
    for (step, pair) in closest_pairs.iter().enumerate() {
        if step == max_steps {
            break;
        }
        maybe_merge_clusters(&pair, &mut clusters);
    }
    clusters.iter().map(|cluster| cluster.len()).sorted().rev().take(3).product()
}

fn part2(input: &Vec<(usize, usize, usize)>) -> usize {
    let mut clusters: Vec<HashSet<usize>> = (0..input.len()).map(|i| HashSet::from([i as usize])).collect();
    let closest_pairs: Vec<(usize, usize)> = (0..input.len()).tuple_combinations().sorted_by(
        |&(a, b), &(c, d)| distance_squared(&input[a], &input[b]).cmp(&distance_squared(&input[c], &input[d]))
    ).collect();
    for pair in closest_pairs {
        maybe_merge_clusters(&pair, &mut clusters);
        if clusters.len() == 1 {
            return input[pair.0].0 * input[pair.1].0;
        }
    }
    panic!("We should not have reached this point.")
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
