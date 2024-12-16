use super::utils::lines_from_file;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static FILEPATH: &str = "inputs/day8.txt";

/// Given a map of frequencies to their antenna locations, return the distinct locations
/// of all antinodes across all frequencies.
fn antinode_locations(
    antenna_locs: &HashMap<char, HashSet<(usize, usize)>>,
    grid: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    antenna_locs
        .iter()
        .map(|(_, locs)| {
            // Collect the pairwise combinations of antenna locations for the current frequency
            let loc_pairs: Vec<((usize, usize), (usize, usize))> = locs
                .iter()
                .cloned()
                .combinations(2)
                .map(|loc_pair| (loc_pair[0].clone(), loc_pair[1].clone()))
                .collect();

            // For each pair of antenna locations of the same frequency, trace the vector between
            // them and add it beyond each endpoint to determine potential antinode locations,
            // filtering out those that are out of bounds
            loc_pairs
                .into_iter()
                .map(|((i_0, j_0), (i_1, j_1))| {
                    [
                        (
                            (i_0, j_0),                                                 // Endpoint 0
                            (i_0 as isize - i_1 as isize, j_0 as isize - j_1 as isize), // Step direction 0
                        ),
                        (
                            (i_1, j_1),                                                 // Endpoint 1
                            (i_1 as isize - i_0 as isize, j_1 as isize - j_0 as isize), // Step direction 1
                        ),
                    ]
                    .into_iter()
                    .filter_map(|(loc, step)| try_step(loc, step, grid))
                })
                .flatten() // Flatten antinode locations across all pairwise combos for the current frequency
        })
        .flatten() // Flatten antinode locations across all frequencies
        .collect()
}

/// Return a map of each frequency to the set of locations of the associated antennas.
fn antenna_locations(grid: &Vec<Vec<char>>) -> HashMap<char, HashSet<(usize, usize)>> {
    let mut locations = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                locations
                    .entry(c)
                    .or_insert_with(HashSet::new)
                    .insert((i, j));
            }
        }
    }
    locations
}

/// Return the coordinates of the hypothetical result of taking the given step from
/// the provided starting point. Return None if the step would be out of bounds.
fn try_step(
    start: (usize, usize),
    step: (isize, isize),
    grid: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    match (
        TryInto::<usize>::try_into(start.0 as isize + step.0),
        TryInto::<usize>::try_into(start.1 as isize + step.1),
    ) {
        (Ok(i), Ok(j)) => {
            if i < grid.len() && j < grid[0].len() {
                Some((i, j))
            } else {
                // At least one index is too high, out of bounds
                None
            }
        }
        _ => None, // At least one index is negative, out of bounds
    }
}

fn get_grid() -> Vec<Vec<char>> {
    lines_from_file(FILEPATH)
        .expect(&format!("Input file {FILEPATH} should exist"))
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve_part_1() {
    let grid = get_grid();
    let antenna_locs = antenna_locations(&grid);
    let antinode_locs = antinode_locations(&antenna_locs, &grid);
    println!("{:?}", antenna_locs);
    let num_antinode_locs = antinode_locs.len();
    println!("Number of unique locations that contain an antinode: {num_antinode_locs}")
}

pub fn solve_part_2() {
    println!("")
}
