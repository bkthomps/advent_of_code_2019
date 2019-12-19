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
                1 => {
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
                    values[element as usize] = lhs + rhs;
                    index += 4;
                }
                2 => {
                    let mut lhs = values[index + 1];
                    if one == 0 {
                        lhs = values[lhs as usize];
                    }
                    let mut rhs = values[index + 2];
                    if two == 0 {
                        rhs = values[rhs as usize];
                    }
                    let mut element = index as i32 + 3;
                    if three == 0 {
                        element = values[element as usize];
                    }
                    values[element as usize] = lhs * rhs;
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
