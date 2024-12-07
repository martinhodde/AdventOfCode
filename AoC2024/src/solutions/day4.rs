use std::collections::HashMap;
use super::utils::lines_from_file;

trait Searchable {
    fn num_matches_from_pt(&self, coords: (usize, usize), grid: &Vec<Vec<char>>) -> u32;
}

/// Part 1
struct XmasSearch {
    start: char,  // The first letter of the searched word
    
    end: char,  // The final letter of the searched word
    
    seq: HashMap<char, char>,  // Sequence for crossword search
}

impl XmasSearch {
    // Define each direction in which the word can be spelled
    pub const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
}

/// Part 2
struct XMASSearch {
    center: char,  // The middle character of an X

    wings: Vec<char>,  // The two characters that comprise the wings of an X
}

impl XMASSearch {
    // Define each direction in the X pattern
    pub const TOP_LEFT: (isize, isize) = (-1, -1);
    pub const TOP_RIGHT: (isize, isize) = (-1, 1);
    pub const BOTTOM_LEFT: (isize, isize) = (1, -1);
    pub const BOTTOM_RIGHT: (isize, isize) = (1, 1);

    pub const DIRECTIONS: [(isize, isize); 4] = [
        XMASSearch::TOP_LEFT,
        XMASSearch::TOP_RIGHT,
        XMASSearch::BOTTOM_LEFT,
        XMASSearch::BOTTOM_RIGHT,
    ];
}

impl Searchable for XmasSearch {
    /// If a start character is detected at the provided coordinates in the grid, draw a line outward
    /// in every possible direction and check for the correct sequence of characters.
    fn num_matches_from_pt(&self, coords: (usize, usize), grid: &Vec<Vec<char>>) -> u32 {
        if grid[coords.0][coords.1] != self.start {
            return 0;
        }

        let mut num_matches: u32 = 0;
        for (i_dir, j_dir) in XmasSearch::DIRECTIONS.iter() {
            let (mut i, mut j) = coords;
            loop {
                if grid[i][j] == self.end {
                    // We have found the desired word if the final character is reached
                    num_matches += 1;
                    break;
                }
                
                // Take a step in the current direction
                let (i_next, j_next) = (i as isize + i_dir, j as isize + j_dir);

                // Break out of the loop if next char is out of bounds or the wrong letter
                match (TryInto::<usize>::try_into(i_next), TryInto::<usize>::try_into(j_next)) {
                    (Ok(i_val), Ok(j_val)) => {
                        // Check if the new index extends beyond the grid and if the
                        // new character is not the next in the sequence
                        if i_val >= grid.len() || j_val >= grid[0].len() || grid[i_val][j_val] != self.seq[&grid[i][j]] {
                            break;
                        }

                        (i, j) = (i_val, j_val);
                    },
                    _ => break,  // At least one index is negative, therefore out of bounds
                }
            }
        }

        num_matches
    }
}

impl Searchable for XMASSearch {
    /// If a center character is detected at the provided coordinates in the grid, draw an X outward
    /// and check for the correct distribution of wing characters.
    fn num_matches_from_pt(&self, coords: (usize, usize), grid: &Vec<Vec<char>>) -> u32 {
        let (i, j) = coords;
        if grid[i][j] != self.center {
            return 0;
        }

        XMASSearch::DIRECTIONS.iter().all(|(i_dir, j_dir)| {
            // Take a step in the current direction
            let (i_next, j_next) = (i as isize + i_dir, j as isize + j_dir);

            // Return false and short-circuit if any char in the X is out of bounds or the wrong letter
            match (TryInto::<usize>::try_into(i_next), TryInto::<usize>::try_into(j_next)) {
                (Ok(i_val), Ok(j_val)) => {
                    // Check if the new index is within the bounds of the grid and if the character
                    // in the current X wing is valid
                    if i_val >= grid.len() || j_val >= grid[0].len() || !self.wings.contains(&grid[i_val][j_val]) {
                        return false;
                    }

                    // Ensure the opposite wing character is not equal to that of the current wing
                    match (*i_dir, *j_dir) {
                        XMASSearch::TOP_LEFT => if i + 1 >= grid.len() || j + 1 >= grid[0].len() { false } else {
                            grid[i_val][j_val] != grid[i + 1][j + 1]
                        },
                        XMASSearch::TOP_RIGHT => if i + 1 >= grid.len() { false } else {
                            grid[i_val][j_val] != grid[i + 1][j - 1]
                        },
                        // The TOP_LEFT and TOP_RIGHT match arms already compare against the
                        // BOTTOM_LEFT and BOTTOM_RIGHT characters, so we default to true here
                        _ => true,
                    }
                },
                _ => false,  // At least one index is negative, therefore out of bounds
            }
        }) as u32
    }
}

fn num_matches_in_grid(match_fn: impl Fn((usize, usize), &Vec<Vec<char>>) -> u32) -> u32 {
    let grid = &get_grid();

    // Take the cross product of the grid index ranges
    let grid_range: Vec<(usize, usize)> = (0..grid.len())
        .flat_map(|i| (0..grid[0].len()).map(move |j| (i, j)))
        .collect();

    // Sum matches over all grid indices
    grid_range.into_iter().map(|coords| match_fn(coords, grid)).sum()
}

fn get_grid() -> Vec<Vec<char>> {
    let filepath = "inputs/day4.txt";
    lines_from_file(filepath).expect(&format!("Input file {filepath} should exist")).into_iter()
        .map(|line: String| line.chars().collect())
        .collect()
}

pub fn solve_part_1() {
    let search = XmasSearch {
        start: 'X',
        end: 'S',
        seq: HashMap::from([
            ('X', 'M'),
            ('M', 'A'),
            ('A', 'S'),
        ]),
    };

    let num_matches = num_matches_in_grid(|coords, grid| search.num_matches_from_pt(coords, grid));
    println!("Number of times XMAS appears: {num_matches}")
}

pub fn solve_part_2() {
    let search = XMASSearch {
        center: 'A',
        wings: vec!['M', 'S'],
    };

    let num_matches = num_matches_in_grid(|coords, grid| search.num_matches_from_pt(coords, grid));
    println!("Number of times X-MAS appears: {num_matches}")
}
