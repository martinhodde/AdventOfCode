use super::utils::lines_from_file;

const FILEPATH: &str = "inputs/day11.txt";

/// Produce the stone engravings that result from performing the specified number of blinks,
/// given the intial vector of stone engravings.
fn simulate_blinks(stones: &Vec<u64>, num_blinks: u64) -> Vec<u64> {
    let mut new_stones: Vec<u64> = stones.clone();

    for _ in 0..num_blinks {
        let mut curr_stones: Vec<u64> = Vec::new();
        for stone in new_stones {
            // Compute the number of digits in the current stone engraving
            let digit_count = (stone as f64).log10().floor() as u32 + 1;

            if stone == 0 {
                // If the stone is engraved with a 0, replace it with a 1
                curr_stones.push(1);
            } else if digit_count % 2 == 0 {
                // If the stone is engraved with a number that has an even number of digits,
                // replace it with two stones. The left half of the digits are engraved on the
                // new left stone, and the right half of the digits are engraved on the new
                // right stone (the new numbers do not keep extra leading zeroes)
                let divisor = 10u64.pow(digit_count / 2);
                curr_stones.push(stone / divisor);
                curr_stones.push(stone % divisor);
            } else {
                // If none of the other rules apply, the stone is replaced by a new stone:
                // the old stone's number multiplied by 2024
                curr_stones.push(stone * 2024);
            }
        }
        new_stones = curr_stones;
    }

    new_stones
}

fn get_stones() -> Vec<u64> {
    lines_from_file(FILEPATH)
        .expect(&format!("Input file {FILEPATH} should exist"))
        .get(0)
        .expect(&format!("Input file {FILEPATH} should have contents"))
        .split(" ")
        .map(|s| s.parse().expect("Cannot parse string to u64"))
        .collect()
}

pub fn solve_part_1() {
    let stones = get_stones();
    let stones_after_blinking = simulate_blinks(&stones, 25).len();
    println!("Number of stones after blinking 25 times: {stones_after_blinking}");
}

pub fn solve_part_2() {
    let stones = get_stones();
    let stones_after_blinking = simulate_blinks(&stones, 75).len();
    println!("Number of stones after blinking 75 times: {stones_after_blinking}");
}
