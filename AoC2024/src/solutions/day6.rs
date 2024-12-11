use super::utils::lines_from_file;
use std::collections::HashSet;

static FILEPATH: &str = "inputs/day6.txt";

/// Return a set of all the grid coordinates visited by the guard, assuming that she
/// takes a right turn each time she encounters an obstacle.
fn get_pos_visited(start: (usize, usize), grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::from([start]);
    let mut pos = start;
    let mut step = (-1, 0);

    while let Ok((i_next, j_next)) = try_step(pos, step, grid) {
        if grid[i_next][j_next] == '#' {
            // Take a 90-degree clockwise turn when an obstacle is encountered
            step = (step.1, -step.0);
        } else {
            // Commit to the step and mark new position as visited
            pos = (i_next, j_next);
            visited.insert(pos);
        }
    }

    visited
}

/// Return the coordinates of the hypothetical result of taking the given step from
/// the provided starting point. Error if the step would be out of bounds.
fn try_step(
    start: (usize, usize),
    step: (isize, isize),
    grid: &Vec<Vec<char>>,
) -> Result<(usize, usize), &str> {
    let (i, j) = start;
    let (i_s, j_s) = step;

    // Probe in the provided direction
    let (i_next, j_next) = (i as isize + i_s, j as isize + j_s);
    match (
        TryInto::<usize>::try_into(i_next),
        TryInto::<usize>::try_into(j_next),
    ) {
        (Ok(i_val), Ok(j_val)) => {
            if i_val < grid.len() && j_val < grid[0].len() {
                Ok((i_val, j_val))
            } else {
                Err("At least one index is too high, therefore out of bounds")
            }
        }
        _ => Err("At least one index is negative, therefore out of bounds"),
    }
}

/// Return the grid coordinates of the starting position, at which point
/// the guard is facing up. Error if no starting point is found.
fn find_start_pt(grid: &Vec<Vec<char>>) -> Result<(usize, usize), &str> {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                return Ok((i, j));
            }
        }
    }
    Err("Starting point not found in grid")
}

fn get_grid() -> Vec<Vec<char>> {
    lines_from_file(FILEPATH)
        .expect(&format!("Input file {FILEPATH} should exist"))
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve_part_1() {
    let grid = &get_grid();
    let start = find_start_pt(grid).unwrap();
    let num_visited = get_pos_visited(start, grid).len();
    println!("Number of distinct positions visited by guard: {num_visited}")
}

pub fn solve_part_2() {
    println!("")
}
