pub mod puzzle_1 {
    use std::collections::HashSet;
    use std::cmp;
    use std::i32;

    #[derive(PartialEq, Eq, Hash)]
    struct Point {
        x: i32,
        y: i32,
    }

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-3.txt", "\n");
        let first = inputs[0].split(",");
        let second = inputs[1].split(",");
        let mut points = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        for n in first {
            let num: i32 = n[1..].parse().expect("Could not cast to int");
            match &n[0..1] {
                "U" => {
                    for i in y + 1..y + num + 1 {
                        points.insert(Point { x, y: i });
                    }
                    y += num;
                }
                "D" => {
                    for i in y - num..y {
                        points.insert(Point { x, y: i });
                    }
                    y -= num;
                }
                "R" => {
                    for i in x + 1..x + num + 1 {
                        points.insert(Point { x: i, y });
                    }
                    x += num;
                }
                "L" => {
                    for i in x - num..x {
                        points.insert(Point { x: i, y });
                    }
                    x -= num;
                }
                _ => println!("Invalid operation")
            }
        }
        x = 0;
        y = 0;
        let mut min = i32::MAX;
        for n in second {
            let num: i32 = n[1..].parse().expect("Could not cast to int");
            match &n[0..1] {
                "U" => {
                    for i in y + 1..y + num + 1 {
                        if points.contains(&Point { x, y: i }) {
                            min = cmp::min(min, x.abs() + i.abs());
                        }
                    }
                    y += num;
                }
                "D" => {
                    for i in y - num..y {
                        if points.contains(&Point { x, y: i }) {
                            min = cmp::min(min, x.abs() + i.abs());
                        }
                    }
                    y -= num;
                }
                "R" => {
                    for i in x + 1..x + num + 1 {
                        if points.contains(&Point { x: i, y }) {
                            min = cmp::min(min, i.abs() + y.abs());
                        }
                    }
                    x += num;
                }
                "L" => {
                    for i in x - num..x {
                        if points.contains(&Point { x: i, y }) {
                            min = cmp::min(min, i.abs() + y.abs());
                        }
                    }
                    x -= num;
                }
                _ => println!("Invalid operation")
            }
        }
        return min;
    }
}

pub mod puzzle_2 {
    use std::collections::HashMap;
    use std::cmp;
    use std::i32;

    #[derive(PartialEq, Eq, Hash)]
    struct Point {
        x: i32,
        y: i32,
    }

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-3.txt", "\n");
        let first = inputs[0].split(",");
        let second = inputs[1].split(",");
        let mut points = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        let mut distance = 0;
        for n in first {
            let num: i32 = n[1..].parse().expect("Could not cast to int");
            match &n[0..1] {
                "U" => {
                    for i in y + 1..y + num + 1 {
                        distance += 1;
                        let cur = Point { x, y: i };
                        if !points.contains_key(&cur) {
                            points.insert(cur, distance);
                        }
                    }
                    y += num;
                }
                "D" => {
                    for i in y - num..y {
                        distance += 1;
                        let cur = Point { x, y: i };
                        if !points.contains_key(&cur) {
                            points.insert(cur, distance);
                        }
                    }
                    y -= num;
                }
                "R" => {
                    for i in x + 1..x + num + 1 {
                        distance += 1;
                        let cur = Point { x: i, y };
                        if !points.contains_key(&cur) {
                            points.insert(cur, distance);
                        }
                    }
                    x += num;
                }
                "L" => {
                    for i in x - num..x {
                        distance += 1;
                        let cur = Point { x: i, y };
                        if !points.contains_key(&cur) {
                            points.insert(cur, distance);
                        }
                    }
                    x -= num;
                }
                _ => println!("Invalid operation")
            }
        }
        x = 0;
        y = 0;
        distance = 0;
        let mut min = i32::MAX;
        for n in second {
            let num: i32 = n[1..].parse().expect("Could not cast to int");
            match &n[0..1] {
                "U" => {
                    for i in y + 1..y + num + 1 {
                        distance += 1;
                        let cur = &Point { x, y: i };
                        if points.contains_key(cur) {
                            let other_distance = *points.get(cur).unwrap();
                            min = cmp::min(min, distance + other_distance);
                        }
                    }
                    y += num;
                }
                "D" => {
                    for i in (y - num..y).rev() {
                        distance += 1;
                        let cur = &Point { x, y: i };
                        if points.contains_key(cur) {
                            let other_distance = *points.get(cur).unwrap();
                            min = cmp::min(min, distance + other_distance);
                        }
                    }
                    y -= num;
                }
                "R" => {
                    for i in x + 1..x + num + 1 {
                        distance += 1;
                        let cur = &Point { x: i, y };
                        if points.contains_key(cur) {
                            let other_distance = *points.get(cur).unwrap();
                            min = cmp::min(min, distance + other_distance);
                        }
                    }
                    x += num;
                }
                "L" => {
                    for i in (x - num..x).rev() {
                        distance += 1;
                        let cur = &Point { x: i, y };
                        if points.contains_key(cur) {
                            let other_distance = *points.get(cur).unwrap();
                            min = cmp::min(min, distance + other_distance);
                        }
                    }
                    x -= num;
                }
                _ => println!("Invalid operation")
            }
        }
        return min;
    }
}
