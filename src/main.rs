mod day_1;
mod day_2;
mod day_3;
mod day_4;

use std::fs;
use std::string;
use std::vec;

fn main() {
    println!("Day 1: {} {}", day_1::puzzle_1::compute(), day_1::puzzle_2::compute());
    println!("Day 2: {} {}", day_2::puzzle_1::compute(), day_2::puzzle_2::compute());
    println!("Day 3: {} {}", day_3::puzzle_1::compute(), day_3::puzzle_2::compute());
    println!("Day 4: {}", day_4::puzzle_1::compute());
}

pub fn get_input(file_name: &str, separator: &str) -> vec::Vec<string::String> {
    let result = fs::read_to_string(format!("{}/{}", "input", file_name));
    let lines = result.expect("Could not load file");
    return lines.split(separator).map(|s| s.to_string()).collect();
}
