pub mod puzzle_1 {
    use std::cmp;
    use std::string;
    use std::i32;
    use std::vec;

    pub fn compute() -> i64 {
        let inputs = crate::get_input("day-10.txt", "\n");
        let asteroid_grid = assemble_grid(inputs);
        let mut max = 0;
        for (y_from, n) in asteroid_grid.iter().enumerate() {
            for (x_from, from) in n.iter().enumerate() {
                let mut count = 0;
                if *from == '#' {
                    for (y_to, m) in asteroid_grid.iter().enumerate() {
                        for (x_to, to) in m.iter().enumerate() {
                            if *to == '#' {
                                let y_diff_total = y_to as i32 - y_from as i32;
                                let x_diff_total = x_to as i32 - x_from as i32;
                                let gcd = gcd(i32::abs(y_diff_total), i32::abs(x_diff_total));
                                let factor = cmp::max(gcd, 1);
                                let y_diff = y_diff_total / factor;
                                let x_diff = x_diff_total / factor;
                                let mut y = y_from;
                                let mut x = x_from;
                                let mut is_blocked = false;
                                y = (y as i32 + y_diff) as usize;
                                x = (x as i32 + x_diff) as usize;
                                while y != y_to || x != x_to {
                                    if asteroid_grid[y][x] == '#' {
                                        is_blocked = true;
                                    }
                                    y = (y as i32 + y_diff) as usize;
                                    x = (x as i32 + x_diff) as usize;
                                }
                                if !is_blocked {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
                max = cmp::max(max, count - 1);
            }
        }
        return max;
    }

    fn assemble_grid(inputs: vec::Vec<string::String>) -> vec::Vec<vec::Vec<char>> {
        let mut asteroid_grid = vec::Vec::new();
        for n in inputs {
            let chars = n.chars();
            let mut arr = vec::Vec::new();
            for c in chars {
                arr.push(c);
            }
            asteroid_grid.push(arr);
        }
        return asteroid_grid;
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 || b == 0 {
            return a + b;
        }
        if a > b {
            return gcd(a - b, b);
        }
        return gcd(a, b - a);
    }
}
