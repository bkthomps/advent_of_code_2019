pub mod puzzle_1 {
    use std::vec;

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-5.txt", ",");
        let mut values: Vec<i32> = Vec::new();
        for input in inputs {
            let val: i32 = input.parse().expect("Could not cast to int");
            values.push(val);
        }
        return calculation(&mut values, 1);
    }

    fn calculation(values: &mut vec::Vec<i32>, mut input: i32) -> i32 {
        let mut index = 0;
        while values[index] != 99 {
            let op_code = values[index] % 100;
            let one = values[index] / 100 % 10;
            let two = values[index] / 1000 % 10;
            let three = values[index] / 10_000 % 10;
            match op_code {
                1 | 2 => {
                    let mut lhs = values[index + 1];
                    if one == 0 {
                        lhs = values[lhs as usize];
                    }
                    let mut rhs = values[index + 2];
                    if two == 0 {
                        rhs = values[rhs as usize];
                    }
                    let mut element: i32 = index as i32 + 3;
                    if three == 0 {
                        element = values[element as usize];
                    }
                    values[element as usize] = if op_code == 1 { lhs + rhs } else { lhs * rhs };
                    index += 4;
                }
                3 => {
                    let element = values[index + 1];
                    values[element as usize] = input;
                    index += 2;
                }
                4 => {
                    let mut element = index as i32 + 1;
                    if one == 0 {
                        element = values[element as usize];
                    }
                    input = values[element as usize];
                    index += 2;
                }
                _ => println!("Invalid operation")
            }
        }
        return input;
    }
}

pub mod puzzle_2 {
    use std::vec;

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-5.txt", ",");
        let mut values: Vec<i32> = Vec::new();
        for input in inputs {
            let val: i32 = input.parse().expect("Could not cast to int");
            values.push(val);
        }
        return calculation(&mut values, 5);
    }

    fn calculation(values: &mut vec::Vec<i32>, mut input: i32) -> i32 {
        let mut index = 0;
        while values[index] != 99 {
            let op_code = values[index] % 100;
            let one = values[index] / 100 % 10;
            let two = values[index] / 1000 % 10;
            let three = values[index] / 10_000 % 10;
            match op_code {
                1 | 2 | 7 | 8 => {
                    let mut lhs = values[index + 1];
                    if one == 0 {
                        lhs = values[lhs as usize];
                    }
                    let mut rhs = values[index + 2];
                    if two == 0 {
                        rhs = values[rhs as usize];
                    }
                    let mut element: i32 = index as i32 + 3;
                    if three == 0 {
                        element = values[element as usize];
                    }
                    if op_code == 1 || op_code == 2 {
                        values[element as usize] = if op_code == 1 { lhs + rhs } else { lhs * rhs };
                    } else if (op_code == 7 && lhs < rhs) || (op_code == 8 && lhs == rhs) {
                        values[element as usize] = 1;
                    } else {
                        values[element as usize] = 0;
                    }
                    index += 4;
                }
                3 => {
                    let element = values[index + 1];
                    values[element as usize] = input;
                    index += 2;
                }
                4 => {
                    let mut element = index as i32 + 1;
                    if one == 0 {
                        element = values[element as usize];
                    }
                    input = values[element as usize];
                    index += 2;
                }
                5 | 6 => {
                    let mut compare = values[index + 1];
                    if one == 0 {
                        compare = values[compare as usize];
                    }
                    let mut new_index = values[index + 2];
                    if two == 0 {
                        new_index = values[new_index as usize];
                    }
                    if (op_code == 5 && compare != 0) || (op_code == 6 && compare == 0) {
                        index = new_index as usize;
                    } else {
                        index += 3;
                    }
                }
                _ => println!("Invalid operation")
            }
        }
        return input;
    }
}
