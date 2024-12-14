use super::utils::lines_from_file;
use std::collections::{HashMap, HashSet};

static FILEPATH: &str = "inputs/day6.txt";

/// Compute the set of distinct coordinates at which a single obstruction can be added
/// to induce a loop in the path of the guard.
fn obstruction_positions(
    path: &HashMap<(usize, usize), HashSet<(isize, isize)>>,
    grid: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    path.iter()
        .map(|(&pos, &ref orientations)| {
            orientations.iter().filter_map(move |&step| {
                // Simulate obstacle in front of guard, then trace path from position
                let new_grid = sim_obstacle_in_front(pos, step, grid);
                let new_path = walk_path(pos, step, &new_grid);
                if has_loop(&new_path, &new_grid) {
                    // If any traced paths are loops, map to obstacle location
                    Some(try_step(pos, step, &new_grid).unwrap())
                } else {
                    None
                }
            })
        })
        .flatten() // Flatten positions from which multiple obstacles may appear
        .collect()
}

/// Return a copy of the provided grid, but with an obstacle inserted in front of the given position
/// according to the step direction. If an obstacle already exists at the location immediately to the
/// right, the obstacle would land directly in front of the guard's initial position, or the step
/// would result in walking out of bounds, return a copy of the original grid unmodified.
fn sim_obstacle_in_front(
    pos: (usize, usize),
    step: (isize, isize),
    grid: &Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    let mut new_grid = grid.clone();
    if let Ok((i_front, j_front)) = try_step(pos, step, grid) {
        if let Ok((i_right, j_right)) = try_step(pos, (step.1, -step.0), grid) {
            let exists_on_right = grid[i_right][j_right] == '#';
            let would_be_on_start = grid[i_front][j_front] == '^';
            let would_block_start = if let Ok((i_below_front, j_below_front)) =
                try_step((i_front, j_front), (1, 0), grid)
            {
                grid[i_below_front][j_below_front] == '^'
            } else {
                false
            };

            // If none of the disqualifying conditions are met, place obstacle in front of position
            if !exists_on_right && !would_be_on_start && !would_block_start {
                new_grid[i_front][j_front] = '#';
            }
        }
    }
    new_grid
}

/// Return whether the provided set of visited positions forms a loop, as indicated
/// by the presence of the designated "loop key" of (grid.len(), grid.len())
fn has_loop(
    path: &HashMap<(usize, usize), HashSet<(isize, isize)>>,
    grid: &Vec<Vec<char>>,
) -> bool {
    path.contains_key(&(grid.len(), grid.len()))
}

/// Return the set of all grid coordinates visited by the guard, starting at the given position
/// and direction, mapped to the guard's direction(s) of travel while visiting each coordinate.
/// This assumes that the guard takes a right turn each time she encounters an obstacle.
///
/// For bookkeeping purposes, if a loop is encountered, the returned set will have a designated
/// "loop key" of (grid.len(), grid.len()) mapped to an empty set.
fn walk_path(
    start_pos: (usize, usize),
    start_step: (isize, isize),
    grid: &Vec<Vec<char>>,
) -> HashMap<(usize, usize), HashSet<(isize, isize)>> {
    let mut visited = HashMap::from([(start_pos, HashSet::from([start_step]))]);
    let mut pos = start_pos;
    let mut step = start_step;

    while let Ok((i_next, j_next)) = try_step(pos, step, grid) {
        if grid[i_next][j_next] == '#' {
            // Take a 90-degree clockwise turn when an obstacle is encountered
            step = (step.1, -step.0);
        } else {
            pos = (i_next, j_next); // Continue traveling in the same direction

            if visited.contains_key(&pos) && visited[&pos].contains(&step) {
                // Position already visited in the same orientation, loop detected...
                // Insert special loop key and exit
                visited.insert((grid.len(), grid.len()), HashSet::default());
                break;
            }
        }

        // Create or update position entry with new orientation
        visited.entry(pos).or_insert_with(HashSet::new).insert(step);
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

/// Return a map of positions visited to the guard's orientation(s) while visiting them,
/// beginning from the starting point found by scanning the input grid.
fn get_path_from_start(grid: &Vec<Vec<char>>) -> HashMap<(usize, usize), HashSet<(isize, isize)>> {
    walk_path(find_start_pt(grid).unwrap(), (-1, 0), grid)
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

fn show_grid(
    grid: &Vec<Vec<char>>,
    visited: &HashMap<(usize, usize), HashSet<(isize, isize)>>,
    obstacles: &HashSet<(usize, usize)>,
) {
    let mut new_grid = grid.clone();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if obstacles.contains(&(i, j)) {
                new_grid[i][j] = 'O';
            } else if visited.contains_key(&(i, j)) {
                new_grid[i][j] = 'X';
            }
        }
    }

    for line in new_grid {
        let chars: String = line.iter().collect();
        println!("{chars}");
    }
}

pub fn solve_part_1() {
    let grid = get_grid();
    let path = get_path_from_start(&grid);
    show_grid(&grid, &path, &HashSet::default());

    let num_visited = path.len();
    println!("Number of distinct positions visited by guard: {num_visited}")
}

pub fn solve_part_2() {
    let grid = get_grid();
    let path = get_path_from_start(&grid);
    // let (i, j) = find_start_pt(&grid).unwrap();
    // path.remove(&(i, j));
    // path.remove(&(i + 1, j));
    let obstacles = obstruction_positions(&path, &grid);
    show_grid(&grid, &path, &obstacles);

    let num_obstructions = obstacles.len();
    println!("Number of possible obstruction positions that create a loop: {num_obstructions}")
}
