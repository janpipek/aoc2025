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
    for line in read_lines(path)
    {
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

fn count_subpaths(current: &Vec<String>, device_map: &HashMap<String, Vec<String>>, target: &String) -> i64 {
    let item = current.last().unwrap();
    if item == target {
        return 1
    }
    let branches = device_map.get(item).unwrap();
    branches.iter().map( |branch| {
        if current.contains(branch) { 0 } else {
            let mut new = current.clone();
            new.push(branch.clone());
            count_subpaths(&new, device_map, target)
        }
    }).sum()
}

fn count_subpaths_including(current: &Vec<String>, device_map: &HashMap<String, Vec<String>>, target: &String, required: &Vec<String>) -> i64 {
    let item = current.last().unwrap();
    if item == target {
        return required.iter().all(|r| current.contains(r)) as i64;
    }
    let branches = device_map.get(item).unwrap();
    branches.iter().map( |branch| {
        if current.contains(branch) { 0 } else {
            let mut new = current.clone();
            new.push(branch.clone());
            count_subpaths_including(&new, device_map, target, required)
        }
    }).sum()
}

fn part2(input: &Input) -> i64 {
    let initial = vec![String::from("svr")];
    let required = vec![String::from("fft"), String::from("dac")];
    let target = String::from("out");
    count_subpaths_including(&initial, &input.device_map, &target, &required)
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
        let input = read_input("data/day11-test2.txt");
        let result = part2(&input);
        assert_eq!(2, result)
    }
}
