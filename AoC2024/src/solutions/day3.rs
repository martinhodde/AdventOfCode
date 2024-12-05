use regex::Regex;
use super::utils::lines_from_file;

/// Find all expressions in the input that match the multiplication pattern,
/// then sum the result of these multiplication programs.
fn mul_sum() -> u32 {
    let lines = &get_input();

    // Capture X and Y in each mul(X,Y) expression, where X and Y are 1-3 digit numbers
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

    re.captures_iter(lines)
        .map(|cap|
            cap.get(1).unwrap().as_str().parse::<u32>().unwrap()
            * cap.get(2).unwrap().as_str().parse::<u32>().unwrap())
        .sum()
}

/// Find all expressions in the input that match the multiplication pattern AND
/// are enabled according to the preceeding do() and don't() instructions, then
/// sum the result of these multiplication programs.
fn mul_sum_enabled() -> u32 {
    let lines = &get_input();

    // Include named capture groups for do() and don't() commands
    let pattern = r"(mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\))|(?<do>do\(\))|(?<do_not>don't\(\))";
    let re = Regex::new(pattern).unwrap();

    let mut sum = 0;
    let mut enabled = true;  // Multiplication is initially enabled
    for cap in re.captures_iter(lines) {
        if enabled {
            if let (Some(op1), Some(op2)) = (cap.name("op1"), cap.name("op2")) {
                sum += op1.as_str().parse::<u32>().unwrap() * op2.as_str().parse::<u32>().unwrap();
            }
        }
        if let Some(_) = cap.name("do") {
            enabled = true;
        } else if let Some(_) = cap.name("do_not") {
            enabled = false;
        }
    }

    sum
}

fn get_input() -> String {
    let filepath = "inputs/day3.txt";
    lines_from_file(filepath).expect(&format!("Input file {filepath} should exist")).join("")
}

pub fn solve_part_1() {
    let sum = mul_sum();
    println!("Sum of multiplication results: {sum}")
}

pub fn solve_part_2() {
    let sum = mul_sum_enabled();
    println!("Sum of only enabled multiplication results: {sum}")
}
