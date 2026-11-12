use aoc2025::read_lines;
use std::collections::HashMap;

fn main() {
    println!("Day 11");
    let input = read_input("data/day11.txt");
    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    let result2 = part2(&input);
    println!("Part 2: {}", result2);
}

struct Input {
    device_map: HashMap<String, Vec<String>>,
}

fn read_input(path: &str) -> Input {
    let mut device_map = HashMap::new();
    for line in read_lines(path) {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            panic!("Invalid input: {}", line);
        }
        let name = parts[0].to_string();
        let outputs: Vec<String> = parts[1].trim().split(' ').map(|s| s.to_string()).collect();
        device_map.insert(name, outputs);
    }
    Input { device_map }
}

fn part1(input: &Input) -> i64 {
    let initial = vec![String::from("you")];
    let target = String::from("out");
    count_subpaths(&initial, &input.device_map, &target)
}

fn count_subpaths(
    current: &Vec<String>,
    device_map: &HashMap<String, Vec<String>>,
    target: &String,
) -> i64 {
    let item = current.last().unwrap();
    if item == target {
        return 1;
    }
    let branches = device_map.get(item).unwrap();
    branches
        .iter()
        .map(|branch| {
            if current.contains(branch) {
                0
            } else {
                let mut new = current.clone();
                new.push(branch.clone());
                count_subpaths(&new, device_map, target)
            }
        })
        .sum()
}

fn find_subpaths(
    current: &Vec<String>,
    device_map: &HashMap<String, Vec<String>>,
    target: &String,
) -> Vec<Vec<String>> {
    let item = current.last().unwrap();

    if item == target {
        let mut new = current.clone();
        new.push(item.clone());
        vec![new]
    } else {
        let mut result: Vec<Vec<String>> = Vec::new();
        match device_map.get(item) {
            Some(branches) => {
                for branch in branches {
                    if !current.contains(branch) {
                        let mut new = current.clone();
                        new.push(branch.clone());
                        result.extend(find_subpaths(&new, device_map, target));
                    }
                }
            }
            None => {}
        }
        result
    }
}

fn find_subpaths_with_exclusion(
    current: &Vec<String>,
    device_map: &HashMap<String, Vec<String>>,
    target: &String,
    mut excluded: Vec<String>,
) -> Vec<Vec<String>> {
    let item = current.last().unwrap();

    if item == target {
        let mut new = current.clone();
        new.push(item.clone());
        vec![new]
    } else {
        let mut result: Vec<Vec<String>> = Vec::new();
        match device_map.get(item) {
            Some(branches) => {
                for branch in branches {
                    if !current.contains(branch) {
                        let mut new = current.clone();
                        new.push(branch.clone());
                        result.extend(find_subpaths_with_exclusion(&new, device_map, target, excluded.clone()));
                    }
                }
            }
            None => {
                // We now that it is not possible to reach branch
                excluded.push(item.clone());
            }
        }
        result
    }
}

fn find_subpaths_many(
    current: &Vec<Vec<String>>,
    device_map: &HashMap<String, Vec<String>>,
    target: &String,
) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    for path in current {
        result.extend(find_subpaths_with_exclusion(&path, device_map, target, Vec::new()));
    }
    result
}

fn part2(input: &Input) -> i64 {
    let fft_dac = find_subpaths(
        &vec![String::from("fft")],
        &input.device_map,
        &String::from("dac"),
    );
    assert_eq!(fft_dac.len(), 0);
    // We continue only with the srv->dac->fft->out route

    let svr_dac = find_subpaths(
        &vec![String::from("svr")],
        &input.device_map,
        &String::from("dac"),
    );
    let svr_dac_fft = find_subpaths_many(&svr_dac, &input.device_map, &String::from("fft"));
    let svr_dac_fft_out = find_subpaths_many(&svr_dac_fft, &input.device_map, &String::from("out"));
    svr_dac_fft_out.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("data/day11-test.txt");
        let result = part1(&input);
        assert_eq!(5, result);
    }

    #[test]
    fn test_part2() {
        // Note: Swapped so that the assert for fft->dac stays
        let input = read_input("data/day11-test2.txt");
        let result = part2(&input);
        assert_eq!(2, result)
    }
}
