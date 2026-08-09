use aoc2025::read_lines;
use regex::Regex;
use itertools::Itertools;

struct Machine {
    lights: Vec<bool>,
    wirings: Vec<Vec<i64>>,
    joltages: Vec<i64>,
}

struct BinaryMachine {
    lights: i64,
    wirings: Vec<i64>
}

type Input = Vec<Machine>;

impl Machine {
    fn to_binary(&self) -> BinaryMachine {
        let mut binary_lights = 0;
        for (i, on) in self.lights.iter().enumerate() {
            if *on {
                binary_lights |= 1 << i;
            }
        }
        let binary_wirings = self.wirings.iter().map(|wiring| {
            let mut binary_wiring: i64 = 0;
            for i in wiring {
                binary_wiring |= 1 << i;
            }
            binary_wiring
        }).collect();
        BinaryMachine {
            lights: binary_lights,
            wirings: binary_wirings,
        }
    }
}

fn to_binary_machines(input: &Input) -> Vec<BinaryMachine> {
    input.iter().map(|m| m.to_binary()).collect()
}

fn read_input(path: &str) -> Input {
    let lines = read_lines(path);
    let pattern = Regex::new(r"\[(.+)\](.+)\{(.+)\}").unwrap();

    lines.iter().map(|line|
        {
            let captures = pattern.captures(line).unwrap();
            let l_part = captures.get(1).unwrap().as_str();
            let w_part = captures.get(2).unwrap().as_str();
            let j_part = captures.get(3).unwrap().as_str();
            Machine {
                lights: l_part.chars().map(|c| c == '#').collect(),
                wirings: w_part.split_whitespace().map(
                    |w| w[1..(w.len() - 1)].split(',').map(
                        |w_i| w_i.parse::<i64>().unwrap()
                    ).collect()
                ).collect(),
                joltages: j_part.split(',').map(|j| j.parse::<i64>().unwrap()).collect(),
            }
        }
    ).collect()
}

fn main() {
    println!("Day 10");
    let input = read_input("data/day10.txt");
    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    let result2 = part2(&input);
    println!("Part 2: {}", result2);
}

fn min_buttons(binary_machine: &BinaryMachine) -> i64 {
    for button_count in 1..binary_machine.wirings.len() {
        for combination in binary_machine.wirings.iter().combinations(button_count) {
            let mut button_result: i64 = 0;
            for button in combination {
                button_result ^= button
            }
            if button_result == binary_machine.lights {
                return button_count as i64
            }
        }
    }
    panic!("Cannot find min buttons")
}

fn part1(input: &Input) -> i64 {
    input.iter().map(|machine| {
        min_buttons(&machine.to_binary())
    }).sum()
}

fn part2(input: &Input) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input("data/day10-test.txt");
        let result = part1(&input);
        assert_eq!(7, result);
    }

    #[test]
    fn test_part2() {
        let input = read_input("data/day10-test.txt");
        let result = part2(&input);
    }
}
