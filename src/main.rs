use std::fs;

use aoc_6;

fn main() {
    part_1();
}

fn part_1() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let result = aoc_6::process_part_1(&input, 4);
    println!("Part 1: {}", result.unwrap());
}
