use aoc2025::read_lines;

struct Device {
    name: String,
    outputs: Vec<String>,
}

type Input = Vec<Device>;

fn read_input(path: &str) -> Input {
    read_lines(path).iter().map(
        |line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                panic!("Invalid input: {}", line);
            }
            Device {
                name: parts[0].to_string(),
                outputs: parts[1].trim().split(' ').map(|s| s.to_string()).collect(),
            }
        }
    ).collect()
}

fn main() {
    println!("Day 11");
    let input = read_input("data/day11.txt");
    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    let result2 = part2(&input);
    println!("Part 2: {}", result2);
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
    fn test_part1() {
        let input = read_input("data/day11-test.txt");
        let result = part1(&input);
        assert_eq!(5, result);
    }

    #[test]
    fn test_part2() {
        let input = read_input("data/day11-test.txt");
        let result = part2(&input);
        // assert_eq!(0, result)
    }
}