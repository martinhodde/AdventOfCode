use super::utils::{lines_from_file, try_step};
use std::collections::HashSet;

const FILEPATH: &str = "inputs/day10.txt";
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

/// Compute the sum of the scores of all trailheads in the topographic trail map.
fn trailhead_score_sum(trail_map: &Vec<Vec<u8>>) -> u32 {
    trail_map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &height)| {
                // Each trailhead starts at height 0
                if height == 0 {
                    Some(trail_score((i, j), trail_map, &mut HashSet::new()))
                } else {
                    None
                }
            })
        })
        .sum()
}

/// Return the number of 9-height positions reachable from the given coordinates.
fn trail_score(
    coords: (usize, usize),
    trail_map: &Vec<Vec<u8>>,
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    let (i, j) = coords;
    if !visited.contains(&coords) && trail_map[i][j] == 9 {
        return 1;
    }

    let mut score_sum = 0;
    for dir in DIRECTIONS {
        if let Some((i_next, j_next)) = try_step(coords, dir, trail_map) {
            // Add to the running score if the current position has not been visited through an
            // alternate trail and it has a height of exactly one more than the previous position
            if !visited.contains(&(i_next, j_next))
                && trail_map[i_next][j_next] == trail_map[i][j] + 1
            {
                score_sum += trail_score((i_next, j_next), trail_map, visited);
                visited.insert((i_next, j_next));
            }
        }
    }
    score_sum
}

fn get_trail_map() -> Vec<Vec<u8>> {
    lines_from_file(FILEPATH)
        .expect(&format!("Input file {FILEPATH} should exist"))
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

pub fn solve_part_1() {
    let trail_map = get_trail_map();
    let sum = trailhead_score_sum(&trail_map);
    println!("Sum of the scores of all trailheads on topographic map: {sum}");
}
