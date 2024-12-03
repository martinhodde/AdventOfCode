use super::utils::lines_from_file;

fn num_safe_records(tolerate_bad_level: bool) -> u32 {
    let filepath = "inputs/day2.txt";
    let lines = lines_from_file(filepath).expect(&format!("Input file {filepath} should exist"));

    let mut num_safe: u32 = 0;
    for line in lines {
        let report: Vec<i32> = line.split_whitespace().map(|v| v.parse().unwrap()).collect();
        if is_safe(&report) {
            num_safe += 1;
        } else if tolerate_bad_level {
            // If we have opted to tolerate a single bad level in each report, check for safety
            // for any possible skipped level.
            num_safe += (0..report.len()).any(|i: usize|
                is_safe(
                    &report.iter().take(i)  // Take all elements up to i
                    .chain(report.iter().skip(i + 1))  // Chain together with elements beyond i
                    .map(|&n| n)  // Dereference
                    .collect()
                )
            ) as u32;
        }
    }

    num_safe
}

/// A report is safe according to the following rules:
///  - The levels are either all increasing or all decreasing.
///  - Any two adjacent levels differ by at least one and at most three.
fn is_safe(report: &Vec<i32>) -> bool {
    is_increasing_safely(report) || is_decreasing_safely(report)
}

fn is_increasing_safely(report: &Vec<i32>) -> bool {
    report.iter().zip(report.iter().skip(1)).all(|(a, b)| a < b && (1..=3).contains(&(b - a))) 
}

fn is_decreasing_safely(report: &Vec<i32>) -> bool {
    report.iter().zip(report.iter().skip(1)).all(|(a, b)| a > b && (1..=3).contains(&(a - b))) 
}

pub fn solve_part_1() {
    let num_safe = num_safe_records(false);
    println!("Number of safe reports: {num_safe}")
}

pub fn solve_part_2() {
    let num_safe = num_safe_records(true);
    println!("Number of safe reports: {num_safe}")
}
