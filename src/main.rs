mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;

use std::fs;
use std::string;
use std::thread;
use std::vec;

fn main() {
    let mut threads: vec::Vec<thread::JoinHandle<i64>> = vec::Vec::new();
    threads.push(thread::spawn(|| day_1::puzzle_1::compute() as i64));
    threads.push(thread::spawn(|| day_1::puzzle_2::compute() as i64));
    threads.push(thread::spawn(|| day_2::puzzle_1::compute() as i64));
    threads.push(thread::spawn(|| day_2::puzzle_2::compute() as i64));
    threads.push(thread::spawn(|| day_3::puzzle_1::compute() as i64));
    threads.push(thread::spawn(|| day_3::puzzle_2::compute() as i64));
    threads.push(thread::spawn(|| day_4::puzzle_1::compute() as i64));
    threads.push(thread::spawn(|| day_4::puzzle_2::compute() as i64));
    threads.push(thread::spawn(|| day_5::puzzle_1::compute() as i64));
    threads.push(thread::spawn(|| day_5::puzzle_2::compute() as i64));
    threads.push(thread::spawn(|| day_6::puzzle_1::compute() as i64));
    threads.push(thread::spawn(|| day_6::puzzle_2::compute() as i64));
    threads.push(thread::spawn(|| day_7::puzzle_1::compute() as i64));
    threads.push(thread::spawn(|| day_7::puzzle_2::compute() as i64));
    threads.push(thread::spawn(|| day_8::puzzle_1::compute() as i64));
    threads.push(thread::spawn(|| day_8::puzzle_2::compute(false) as i64));
    threads.push(thread::spawn(|| day_9::puzzle_1_and_2::compute(1)));
    threads.push(thread::spawn(|| day_9::puzzle_1_and_2::compute(2)));
    threads.push(thread::spawn(|| day_10::puzzle_1::compute()));
    threads.push(thread::spawn(|| day_10::puzzle_1::compute()));
    for i in (0..threads.len() / 2).rev() {
        let puzzle_2 = threads.remove(threads.len() - 1).join().unwrap();
        let puzzle_1 = threads.remove(threads.len() - 1).join().unwrap();
        println!("Day {}: {} {}", i + 1, puzzle_1, puzzle_2);
    }
}

pub fn get_input(file_name: &str, separator: &str) -> vec::Vec<string::String> {
    let result = fs::read_to_string(format!("{}/{}", "input", file_name));
    let lines = result.expect("Could not load file");
    return lines.split(separator).map(|s| s.to_string()).collect();
}
