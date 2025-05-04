use super::utils::lines_from_file;

const FILEPATH: &str = "inputs/day9.txt";

/// Compute the checksum of the sequence of file ID numbers.
fn checksum(filesystem: &Vec<i32>) -> u64 {
    filesystem
        .into_iter()
        .enumerate()
        .map(|(idx, &file_id)| idx as u64 * file_id as u64)
        .sum()
}

/// From left to right, fill in all free disk blocks with the contents of the rightmost
/// occupied disk blocks, producing a compacted version of the filesystem.
fn compact_filesystem(file_blocks: &Vec<i32>) -> Vec<i32> {
    // Form a queue out of the occupied blocks in increasing order of their file IDs
    // to be used to populate the free spaces
    let mut occupied_blocks: Vec<i32> = file_blocks.iter().copied().filter(|&id| id >= 0).collect();
    let num_occupied_blocks = occupied_blocks.len();

    let mut rearranged_files: Vec<i32> = Vec::new();
    for (i, &block) in file_blocks.iter().enumerate() {
        if i == num_occupied_blocks {
            break;
        } else if block == -1 {
            // Move rightmost file block to next open space
            rearranged_files.push(occupied_blocks.pop().unwrap());
        } else {
            // Keep file block in place if already associated with a file ID
            rearranged_files.push(block);
        }
    }

    rearranged_files
}

/// Convert the disk map into a vector representing the allocation of disk blocks,
/// which consists of alternating sequences of postitive integer file IDs, and -1's
/// indicating free blocks of space.
fn get_file_blocks(disk_map: &Vec<char>) -> Vec<i32> {
    let mut file_blocks: Vec<i32> = Vec::new();
    for (i, &disk_val) in disk_map.iter().enumerate() {
        if i % 2 == 0 {
            // Even disk map entries store the number of blocks for file with ID i/2
            file_blocks.extend(&vec![i as i32 / 2; char_to_digit(disk_val)]);
        } else {
            // Odd disk map entries store the number of free blocks that follow
            file_blocks.extend(&vec![-1; char_to_digit(disk_val)]);
        }
    }

    file_blocks
}

fn char_to_digit(c: char) -> usize {
    c as usize - '0' as usize
}

fn get_disk_map() -> Vec<char> {
    lines_from_file(FILEPATH)
        .expect(&format!("Input file {FILEPATH} should exist"))
        .get(0)
        .expect(&format!("Input file {FILEPATH} should have contents"))
        .chars()
        .collect()
}

pub fn solve_part_1() {
    let disk_map = get_disk_map();
    let file_blocks = get_file_blocks(&disk_map);
    let filesystem = compact_filesystem(&file_blocks);
    let checksum = checksum(&filesystem);
    println!("Checksum of the compacted filesystem: {checksum}")
}

pub fn solve_part_2() {
    // let disk_map = get_disk_map();
    println!()
}
