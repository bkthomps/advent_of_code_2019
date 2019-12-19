pub mod puzzle_1 {
    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-4.txt", "-");
        let first: i32 = inputs[0].parse().expect("Could not cast to int");
        let second: i32 = inputs[1].parse().expect("Could not cast to int");
        let mut count = 0;
        for i in first..second + 1 {
            let chars: Vec<char> = i.to_string().chars().collect();
            if two_adjacent_exists(&chars) && !decreases(&chars) {
                count += 1;
            }
        }
        return count;
    }

    fn two_adjacent_exists(chars: &Vec<char>) -> bool {
        for i in 0..chars.len() - 1 {
            if chars[i] == chars[i + 1] {
                return true;
            }
        }
        return false;
    }

    fn decreases(chars: &Vec<char>) -> bool {
        for i in 0..chars.len() - 1 {
            if chars[i] > chars[i + 1] {
                return true;
            }
        }
        return false;
    }
}

pub mod puzzle_2 {
    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-4.txt", "-");
        let first: i32 = inputs[0].parse().expect("Could not cast to int");
        let second: i32 = inputs[1].parse().expect("Could not cast to int");
        let mut count = 0;
        for i in first..second + 1 {
            let num = i.to_string();
            if exactly_two_adjacent_exists(&num) && !decreases(&num) {
                count += 1;
            }
        }
        return count;
    }

    fn exactly_two_adjacent_exists(num: &str) -> bool {
        let chars: Vec<char> = format!("_{}_", num).chars().collect();
        for i in 0..chars.len() - 2 {
            if chars[i] == chars[i + 1] && chars[i] != chars[i - 1]
                    && chars[i + 1] != chars[i + 2] {
                return true;
            }
        }
        return false;
    }

    fn decreases(num: &str) -> bool {
        let chars: Vec<char> = num.chars().collect();
        for i in 0..chars.len() - 1 {
            if chars[i] > chars[i + 1] {
                return true;
            }
        }
        return false;
    }
}
