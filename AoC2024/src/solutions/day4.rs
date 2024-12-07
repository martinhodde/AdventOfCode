use std::collections::HashMap;
use super::utils::lines_from_file;

struct SearchParams {
    // The first letter of the searched word
    start: char,

    // The final letter of the searched word
    end: char,

    // Adjacency map for crossword search
    adj_map: HashMap<char, char>,

    // Possible search directions
    dirs: Vec<(isize, isize)>
}

fn num_matches_in_grid(search: &SearchParams) -> u32 {
    let grid = &get_grid();

    // Take the cross product of the grid index ranges
    let grid_range: Vec<(usize, usize)> = (0..grid.len())
        .flat_map(|x| (0..grid[0].len()).map(move |y| (x, y)))
        .collect();

    grid_range.into_iter().map(|coords| num_matches_from_pt(coords, grid, search)).sum()
}

fn num_matches_from_pt(coords: (usize, usize), grid: &Vec<Vec<char>>, search: &SearchParams) -> u32 {
    if grid[coords.0][coords.1] != search.start {
        return 0;
    }

    let mut num_matches: u32 = 0;
    for (i_dir, j_dir) in &search.dirs {
        let (mut i, mut j) = coords;
        loop {
            if grid[i][j] == search.end {
                // We have read the whole word if the final character is reached
                num_matches += 1;
                break;
            }

            // Take a step in the current direction
            let (i_next, j_next) = (i as isize + i_dir, j as isize + j_dir);

            // Break out of the loop if next char is out of bounds or the wrong letter
            match (i_next.try_into(), j_next.try_into()) {
                (Ok(i_val), Ok(j_val)) => {
                    let is_idx_high = i_val >= grid.len() || j_val >= grid[0].len();
                    let is_wrong_nxt_char = grid.get(i_val)
                        .and_then(|row: &Vec<char>| row.get(j_val))
                        .map_or(false, |&val| val != search.adj_map[&grid[i][j]]);
                    if is_idx_high || is_wrong_nxt_char {
                        break;
                    }
                    (i, j) = (i_val, j_val);
                },
                _ => break,  // At least one index is negative
            }
        }
    }

    num_matches
}

fn get_grid() -> Vec<Vec<char>> {
    let filepath = "inputs/day4.txt";
    lines_from_file(filepath).expect(&format!("Input file {filepath} should exist")).into_iter()
        .map(|line: String| line.chars().collect())
        .collect()
}

pub fn solve_part_1() {
    let search = SearchParams {
        start: 'X',
        end: 'S',
        adj_map: HashMap::from([
            ('X', 'M'),
            ('M', 'A'),
            ('A', 'S'),
        ]),
        dirs: vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
    };
    let num_matches = num_matches_in_grid(&search);
    println!("Number of times XMAS appears: {num_matches}")
}

pub fn solve_part_2() {
    println!()
}
