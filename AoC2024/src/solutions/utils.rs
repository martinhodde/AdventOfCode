use crate::solutions;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub fn get_solver_fn(idx: (u32, u32)) -> Result<fn(), &'static str> {
    match idx {
        (1, 1) => Ok(solutions::day1::solve_part_1),
        (1, 2) => Ok(solutions::day1::solve_part_2),
        (2, 1) => Ok(solutions::day2::solve_part_1),
        (2, 2) => Ok(solutions::day2::solve_part_2),
        (3, 1) => Ok(solutions::day3::solve_part_1),
        (3, 2) => Ok(solutions::day3::solve_part_2),
        (4, 1) => Ok(solutions::day4::solve_part_1),
        (4, 2) => Ok(solutions::day4::solve_part_2),
        (5, 1) => Ok(solutions::day5::solve_part_1),
        (5, 2) => Ok(solutions::day5::solve_part_2),
        (6, 1) => Ok(solutions::day6::solve_part_1),
        (6, 2) => Ok(solutions::day6::solve_part_2),
        (7, 1) => Ok(solutions::day7::solve_part_1),
        (7, 2) => Ok(solutions::day7::solve_part_2),
        _ => Err("Solver not yet implemented!"),
    }
}
