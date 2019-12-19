pub mod puzzle_1 {
    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-1.txt", "\n");
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

pub mod puzzle_2 {
    use std::cmp;

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-1.txt", "\n");
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
