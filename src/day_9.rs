pub mod puzzle_1 {
    use std::vec;

    const POSITION_MODE: i64 = 0;
    const PARAMETER_MODE: i64 = 1;
    const RELATIVE_MODE: i64 = 2;

    pub fn compute() -> i64 {
        let inputs = crate::get_input("day-9.txt", ",");
        let mut values: Vec<i64> = Vec::new();
        for input in inputs {
            let val = input.parse().expect("Could not cast to int");
            values.push(val);
        }
        return calculation(&mut values, 1);
    }

    fn calculation(values: &mut vec::Vec<i64>, initial_input: i64) -> i64 {
        let mut queue = vec::Vec::new();
        queue.push(initial_input);
        let mut index = 0;
        let mut relative_base = 0;
        while values[index] != 99 {
            let op_code = values[index] % 100;
            let one = values[index] / 100 % 10;
            let two = values[index] / 1000 % 10;
            let three = values[index] / 10_000 % 10;
            match op_code {
                1 | 2 | 7 | 8 => {
                    let lhs = access(values, index + 1, one, relative_base);
                    let rhs = access(values, index + 2, two, relative_base);
                    let result = match op_code {
                        1 => lhs + rhs,
                        2 => lhs * rhs,
                        7 => (lhs < rhs) as i64,
                        8 => (lhs == rhs) as i64,
                        _ => 0
                    };
                    let mut element = get_values(values, index + 3);
                    if three == RELATIVE_MODE {
                        element += relative_base;
                    }
                    set_values(values, element as usize, result);
                    index += 4;
                }
                3 => {
                    let mut element = get_values(values, index + 1);
                    if one == RELATIVE_MODE {
                        element += relative_base;
                    }
                    set_values(values, element as usize, queue.remove(0));
                    index += 2;
                }
                4 => {
                    let value = access(values, index + 1, one, relative_base);
                    queue.push(value);
                    index += 2;
                }
                5 | 6 => {
                    let compare = access(values, index + 1, one, relative_base);
                    let new_index = access(values, index + 2, two, relative_base);
                    if (op_code == 5 && compare != 0) || (op_code == 6 && compare == 0) {
                        index = new_index as usize;
                    } else {
                        index += 3;
                    }
                }
                9 => {
                    relative_base += access(values, index + 1, one, relative_base);
                    index += 2;
                }
                _ => {
                    println!("Invalid operation {}", op_code);
                    return -1;
                }
            }
        }
        return queue[0];
    }

    fn access(values: &vec::Vec<i64>, index: usize, mode: i64, relative_base: i64) -> i64 {
        let result = get_values(values, index);
        return match mode {
            POSITION_MODE => get_values(values, result as usize),
            PARAMETER_MODE => result,
            RELATIVE_MODE => get_values(values, (result + relative_base) as usize),
            _ => {
                println!("Invalid mode {}", mode);
                return -1;
            }
        };
    }

    fn get_values(values: &vec::Vec<i64>, index: usize) -> i64 {
        if index < values.len() {
            return values[index];
        }
        return 0;
    }

    fn set_values(values: &mut vec::Vec<i64>, index: usize, with: i64) {
        if index >= values.len() {
            for _ in 0..index - values.len() + 1 {
                values.push(0);
            }
        }
        values[index] = with;
    }
}
