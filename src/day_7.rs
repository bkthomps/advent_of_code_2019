pub mod puzzle_1 {
    use std::cmp;
    use std::string;
    use std::vec;

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-7.txt", ",");
        let mut max = 0;
        for a in 0..5 {
            let one = computer(&inputs, a, 0);
            for b in 0..5 {
                if b == a {
                    continue;
                }
                let two = computer(&inputs, b, one);
                for c in 0..5 {
                    if c == b || c == a {
                        continue;
                    }
                    let three = computer(&inputs, c, two);
                    for d in 0..5 {
                        if d == c || d == b || d == a {
                            continue;
                        }
                        let four = computer(&inputs, d, three);
                        for e in 0..5 {
                            if e == d || e == c || e == b || e == a {
                                continue;
                            }
                            let five = computer(&inputs, e, four);
                            max = cmp::max(max, five);
                        }
                    }
                }
            }
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
