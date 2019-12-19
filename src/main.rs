use std::fs;
use std::string;
use std::vec;

fn main() {
    let day_1_1 = day_one::puzzle_one::compute();
    let day_1_2 = day_one::puzzle_two::compute();
    println!("Day 1: {} {}", day_1_1, day_1_2);
    let day_2_1 = day_two::puzzle_one::compute();
    let day_2_2 = day_two::puzzle_two::compute();
    println!("Day 2: {} {}", day_2_1, day_2_2);
}

pub fn get_input(file_name: &str, separator: &str) -> vec::Vec<string::String> {
    let result = fs::read_to_string(format!("{}/{}", "input", file_name));
    let lines = result.expect("Could not load file");
    return lines.split(separator).map(|s| s.to_string()).collect();
}

mod day_one {
    pub mod puzzle_one {
        pub fn compute() -> i32 {
            let inputs = crate::get_input("day-1-1.txt", "\n");
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
            let inputs = crate::get_input("day-1-2.txt", "\n");
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

mod day_two {
    pub mod puzzle_one {
        use std::vec;

        pub fn compute() -> usize {
            let inputs = crate::get_input("day-2-1.txt", ",");
            let mut values: Vec<usize> = Vec::new();
            for input in inputs {
                let val: usize = input.parse().expect("Could not cast to int");
                values.push(val);
            }
            values[1] = 12;
            values[2] = 2;
            calculation(&mut values);
            return values[0];
        }

        fn calculation(values: &mut vec::Vec<usize>) {
            let mut index = 0;
            while values[index] != 99 {
                let lhs = values[values[index + 1]];
                let rhs = values[values[index + 2]];
                let element = values[index + 3];
                match values[index] {
                    1 => values[element] = lhs + rhs,
                    2 => values[element] = lhs * rhs,
                    _ => println!("Invalid operation")
                }
                index += 4;
            }
        }
    }

    pub mod puzzle_two {
        use std::vec;

        pub fn compute() -> usize {
            let inputs = crate::get_input("day-2-2.txt", ",");
            let mut values: Vec<usize> = Vec::new();
            for input in inputs {
                let val: usize = input.parse().expect("Could not cast to int");
                values.push(val);
            }
            for i in 0..100 {
                for j in 0..100 {
                    let clone = values.clone();
                    let value = compute_with(clone, i, j);
                    if value == 19690720 {
                        return 100 * i + j;
                    }
                }
            }
            return 0;
        }

        fn compute_with(mut values: vec::Vec<usize>, first: usize, second: usize) -> usize {
            values[1] = first;
            values[2] = second;
            calculation(&mut values);
            return values[0];
        }

        fn calculation(values: &mut vec::Vec<usize>) {
            let mut index = 0;
            while values[index] != 99 {
                let lhs = values[values[index + 1]];
                let rhs = values[values[index + 2]];
                let element = values[index + 3];
                match values[index] {
                    1 => values[element] = lhs + rhs,
                    2 => values[element] = lhs * rhs,
                    _ => println!("Invalid operation")
                }
                index += 4;
            }
        }
    }
}
