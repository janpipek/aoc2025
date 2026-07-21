use std::fs;

// Read all non-empty lines from a file
pub fn read_lines(path: &str) -> Vec<String> {
    let lines = fs::read_to_string(path).expect("Cannot read file.");
    lines.lines().map(|l| String::from(l)).filter(|l| l.trim().len() > 0).collect()
}
