pub mod puzzle_1 {
    use std::vec;

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-2.txt", ",");
        let mut values: Vec<usize> = Vec::new();
        for input in inputs {
            let val: usize = input.parse().expect("Could not cast to int");
            values.push(val);
        }
        values[1] = 12;
        values[2] = 2;
        calculation(&mut values);
        return values[0] as i32;
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

pub mod puzzle_2 {
    use std::vec;

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-2.txt", ",");
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
                    return 100 * i as i32 + j as i32;
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
