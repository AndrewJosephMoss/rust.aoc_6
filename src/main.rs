use std::fs;

use aoc_6;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let result = aoc_6::process_part_1(&input, 4);
    println!("Part 1: {}", result.unwrap());
}

fn part_2() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let result = aoc_6::process_part_1(&input, 14);
    println!("Part 2: {}", result.unwrap());
}
