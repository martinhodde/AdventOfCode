use super::utils::lines_from_file;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static FILEPATH: &str = "inputs/day8.txt";

/// Given a map of frequencies to their antenna locations, return the distinct locations
/// of all antinodes across all frequencies according to the provided antinode location function.
fn antinode_locations(
    antenna_locs: &HashMap<char, HashSet<(usize, usize)>>,
    grid: &Vec<Vec<char>>,
    antinode_fn: fn((usize, usize), (usize, usize), &Vec<Vec<char>>) -> HashSet<(usize, usize)>,
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

            // For each pair of antenna locations of the same frequency, determine the
            // possible antinode locations
            loc_pairs
                .into_iter()
                .map(|(loc1, loc2)| antinode_fn(loc1, loc2, grid))
                .flatten() // Flatten antinodes across all pairwise combos for the current frequency
        })
        .flatten() // Flatten antinodes across all frequencies
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

/// Given two antenna locations of the same frequency, compute the (up to) two possible antinode
/// locations, subject to the bounds of the provided grid.
fn get_antinode_pts(
    loc1: (usize, usize),
    loc2: (usize, usize),
    grid: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    let (i_1, j_1) = (loc1.0 as isize, loc1.1 as isize);
    let (i_2, j_2) = (loc2.0 as isize, loc2.1 as isize);

    // For each of the two endpoints, trace a vector from the other point to itself, then add
    // this vector to the current point to determine potential antinode locations, filtering
    // out those that are out of bounds
    [
        (loc1, (i_1 - i_2, j_1 - j_2)),
        (loc2, (i_2 - i_1, j_2 - j_1)),
    ]
    .into_iter()
    .filter_map(|(loc, step)| try_step(loc, step, grid))
    .collect()
}

/// Given two antenna locations of the same frequency, compute all possible antinode
/// locations, subject to the bounds of the provided grid and taking into account the
/// effects of resonant harmonics.
fn get_antinode_pts_with_resonance(
    loc1: (usize, usize),
    loc2: (usize, usize),
    grid: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    let (i_1, j_1) = (loc1.0 as isize, loc1.1 as isize);
    let (i_2, j_2) = (loc2.0 as isize, loc2.1 as isize);

    // For each of the two endpoints, trace a vector from the other point to itself, then add
    // this vector to the current point repeatedly to determine potential antinode locations,
    // terminating when the vector addition results in stepping out of bounds
    let mut antinode_locs: HashSet<(usize, usize)> = HashSet::from([loc1, loc2]);
    let loc_dirs = [
        (loc1, (i_1 - i_2, j_1 - j_2)),
        (loc2, (i_2 - i_1, j_2 - j_1)),
    ];

    for (start_loc, step) in loc_dirs {
        let mut loc = start_loc;
        while let Some(new_loc) = try_step(loc, step, grid) {
            antinode_locs.insert(new_loc);
            loc = new_loc;
        }
    }

    antinode_locs
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
                None // At least one index is too high, out of bounds
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
    let antinode_locs = antinode_locations(&antenna_locs, &grid, get_antinode_pts);
    let num_antinode_locs = antinode_locs.len();
    println!("Number of unique locations that contain an antinode: {num_antinode_locs}")
}

pub fn solve_part_2() {
    let grid = get_grid();
    let antenna_locs = antenna_locations(&grid);
    let antinode_locs = antinode_locations(&antenna_locs, &grid, get_antinode_pts_with_resonance);
    let num_antinode_locs = antinode_locs.len();
    println!("Number of unique locations that contain an antinode, considering resonant harmonics: {num_antinode_locs}")
}
