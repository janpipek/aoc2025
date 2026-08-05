use std::fs;

fn read_lines() -> Vec<String> {
    let lines = fs::read_to_string("data/day04.txt").expect("Cannot read file.");
    lines
        .lines()
        .map(|l| String::from(l))
        .filter(|l| l.trim().len() > 0)
        .collect()
}

fn read_input() -> Vec<Vec<bool>> {
    let lines = read_lines();
    lines
        .iter()
        .filter(|l| l.len() > 0)
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn main() {
    println!("Day 5");
    part1();
    part2();
}

fn is_pickable(input: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let width = input[0].len();
    let height = input.len();
    if !input[y][x] {
        return false;
    }
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let nx = x as i32 + i;
            let ny = y as i32 + j;
            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                count += input[ny as usize][nx as usize] as u32;
            }
        }
    }
    count < 4
}

fn count_them(input: &Vec<Vec<bool>>) -> i32 {
    let width = input[0].len();
    let height = input.len();
    let mut total_count = 0;
    for x in 0..width {
        for y in 0..height {
            total_count += is_pickable(&input, x, y) as i32;
        }
    }
    total_count
}

fn part1() {
    let input = read_input();
    let total_count = count_them(&input);
    println!("Part 1: {}", total_count);
}

fn part2() {
    println!("Part 2: ---");
    let mut input = read_input();
    let width = input[0].len();
    let height = input.len();
    let mut total_count = 0; // count_them(&input);
    loop {
        let mut current_count = 0;
        for x in 0..width {
            for y in 0..height {
                if input[y][x] {
                    if is_pickable(&input, x, y) {
                        current_count += 1;
                        input[y][x] = false;
                    }
                }
            }
        }
        if current_count == 0 {
            break;
        }
        total_count += current_count;
    }
    println!("Part 2: {}", total_count);
}
