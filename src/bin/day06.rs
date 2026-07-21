use aoc2025::read_lines;

fn main() {
    println!("Day 6");
    part1();
    part2();
}

fn part1() {
    let lines: Vec<String> = read_lines("data/day06.txt");
    let table: Vec<Vec<i64>> = lines[..lines.len()-1].iter().map(|l| l.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect()).collect();
    let operators: Vec<String> = lines[lines.len()-1].split_whitespace().map(|s| String::from(s)).collect();
    let result: i64 = (0..table[0].len()).map(|i| {
        if operators[i] == "*" {
            table.iter().map(|row| row[i]).product::<i64>() as i64
        } else {
            table.iter().map(|row| row[i]).sum::<i64>()
        }
    }).sum();
    println!("Part 1: {result}")
}

fn part2() {
    println!("Part 2: not solved yet.")
}
