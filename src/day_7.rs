pub mod puzzle_1 {
    use std::collections::HashSet;
    use std::cmp;
    use std::string;
    use std::vec;

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-7.txt", ",");
        let mut options = HashSet::new();
        for n in 0..5 {
            options.insert(n);
        }
        return amplify(&inputs, options, 0);
    }

    fn amplify(inputs: &vec::Vec<string::String>, options: HashSet<i32>, input: i32) -> i32 {
        if options.len() == 1 {
            for n in &options {
                return computer(inputs, *n, input);
            }
        }
        let mut max = 0;
        for n in &options {
            let result = computer(inputs, *n, input);
            let mut new_options = options.clone();
            new_options.remove(n);
            max = cmp::max(max, amplify(inputs, new_options, result));
        }
        return max;
    }

    fn computer(inputs: &vec::Vec<string::String>, option: i32, input: i32) -> i32 {
        let mut values: Vec<i32> = Vec::new();
        for input in inputs {
            let val: i32 = input.parse().expect("Could not cast to int");
            values.push(val);
        }
        let mut vec: Vec<i32> = vec::Vec::new();
        vec.push(input);
        vec.push(option);
        return calculation(&mut values, vec);
    }

    fn calculation(values: &mut vec::Vec<i32>, mut input: vec::Vec<i32>) -> i32 {
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
                    values[element as usize] = input.remove(input.len() - 1);
                    index += 2;
                }
                4 => {
                    let mut element = index as i32 + 1;
                    if one == 0 {
                        element = values[element as usize];
                    }
                    input.push(values[element as usize]);
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
        return input[0];
    }
}

pub mod puzzle_2 {
    use std::collections::HashSet;
    use std::cmp;
    use std::vec;

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-7.txt", ",");
        let mut values: Vec<i32> = Vec::new();
        for input in inputs {
            let val: i32 = input.parse().expect("Could not cast to int");
            values.push(val);
        }
        let mut options = HashSet::new();
        for n in 5..10 {
            options.insert(n);
        }
        let settings: vec::Vec<i32> = vec::Vec::new();
        return choose_settings(options, settings, &values);
    }

    fn choose_settings(options: HashSet<i32>, settings: vec::Vec<i32>, values: &Vec<i32>) -> i32 {
        if options.is_empty() {
            return simulate(settings, values);
        }
        let mut max = 0;
        for n in &options {
            let mut new_options = options.clone();
            new_options.remove(n);
            let mut new_settings = settings.clone();
            new_settings.push(*n);
            max = cmp::max(max, choose_settings(new_options, new_settings, &values));
        }
        return max;
    }

    fn simulate(mut settings: vec::Vec<i32>, values: &Vec<i32>) -> i32 {
        let mut amplifiers = [values.clone(), values.clone(), values.clone(), values.clone(), values.clone()];
        let mut index = 0;
        let mut input = 0;
        let mut stop = false;
        let mut last_output = [0; 5];
        let mut addresses = [0; 5];
        while !stop || index != 0 {
            let (output, stopping, address) =
                    calculation(&mut amplifiers[index], settings[index], input, last_output[index],
                                addresses[index]);
            last_output[index] = output;
            input = output;
            stop = stopping;
            addresses[index] = address;
            settings[index] = -1;
            index += 1;
            index %= settings.len();
        }
        return input;
    }

    fn calculation(values: &mut vec::Vec<i32>, mut option: i32, input: i32, last_output: i32,
                   mut index: usize) -> (i32, bool, usize) {
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
                    if option != -1 {
                        let element = values[index + 1];
                        values[element as usize] = option;
                        option = -1;
                    } else {
                        let element = values[index + 1];
                        values[element as usize] = input;
                    }
                    index += 2;
                }
                4 => {
                    let mut element = index as i32 + 1;
                    if one == 0 {
                        element = values[element as usize];
                    }
                    return (values[element as usize], false, index + 2);
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
        return (last_output, true, index);
    }
}
