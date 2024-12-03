use std::collections::BinaryHeap;
use super::utils::lines_from_file;

pub fn solve_part_1() {
    let filepath = "inputs/day1.txt";
    let lines = lines_from_file(filepath).expect(&format!("Input file {filepath} should exist"));

    // Aggregate both input columns in binary heaps to maintain sorted order
    let mut heap1: BinaryHeap<u32> = BinaryHeap::new();
    let mut heap2: BinaryHeap<u32> = BinaryHeap::new();
    for line in lines {
        let vals: Vec<u32> = line.split("   ").map(|s| s.parse().unwrap()).collect();
        heap1.push(vals[0]);
        heap2.push(vals[1]);
    }

    // Convert to vectors
    let vec1 = heap1.into_sorted_vec();
    let vec2 = heap2.into_sorted_vec();

    // Compute the element-wise absolute difference between the two vectors
    let abs_diff_vec: Vec<u32> = vec1.iter().zip(vec2.iter()).map(|(&v1, &v2)| v1.abs_diff(v2)).collect();
    let sum: u32 = abs_diff_vec.into_iter().sum();
    println!("Total distance between lists: {sum}")
}

pub fn solve_part_2() {
    println!("Hello, world!")
}
