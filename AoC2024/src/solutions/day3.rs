use regex::Regex;
use super::utils::lines_from_file;

/// Find all expressions in the input line that match the mul(X,Y) pattern,
/// then sum the result of these multiplication programs.
fn mul_sum(line: &str) -> u32 {
    // Capture X and Y in each mul(X,Y) expression, where X and Y are 1-3 digit numbers
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

    // Find all valid X,Y multiplication operands, multiply them, then take the sum
    re.captures_iter(line)
        .map(|cap| cap.get(1).map_or(0, |m| m.as_str().parse().unwrap())
            * cap.get(2).map_or(0, |m| m.as_str().parse().unwrap()))
        .sum()
}

pub fn solve_part_1() {
    let filepath = "inputs/day3.txt";
    let lines = lines_from_file(filepath).expect(&format!("Input file {filepath} should exist"));
    let sum: u32 = lines.into_iter().map(|line| mul_sum(&line)).sum();
    println!("Sum of multiplication results: {sum}")
}

pub fn solve_part_2() {
    println!("")
}
