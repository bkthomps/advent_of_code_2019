use std::fs;
use std::string;
use std::vec;

fn main() {
    let day_1_1 = day_one::puzzle_one::compute();
    let day_1_2 = day_one::puzzle_two::compute();
    println!("Day 1: {} {}", day_1_1, day_1_2);
}

pub fn get_input(file_name: &str) -> vec::Vec<string::String> {
    let result = fs::read_to_string(format!("{}/{}", "input", file_name));
    let lines = result.expect("Could not load file");
    return lines.split("\n").map(|s| s.to_string()).collect();
}

mod day_one {
    pub mod puzzle_one {
        pub fn compute() -> i32 {
            let inputs = crate::get_input("day-1-1.txt");
            let mut result = 0;
            for input in inputs {
                let add = input.parse().expect("Could not cast to int");
                result += calculation(add);
            }
            return result;
        }

        fn calculation(mass: i32) -> i32 {
            return mass / 3 - 2;
        }
    }

    pub mod puzzle_two {
        use std::cmp;

        pub fn compute() -> i32 {
            let inputs = crate::get_input("day-1-2.txt");
            let mut result = 0;
            for input in inputs {
                let add = input.parse().expect("Could not cast to int");
                result += calculation(add);
            }
            return result;
        }

        fn calculation(mass: i32) -> i32 {
            let needed_fuel = mass / 3 - 2;
            if needed_fuel <= 0 {
                return cmp::max(needed_fuel, 0);
            }
            return needed_fuel + calculation(needed_fuel);
        }
    }
}
