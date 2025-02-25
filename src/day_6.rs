pub mod puzzle_1 {
    use std::collections::HashMap;
    use std::string;
    use std::vec;

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-6.txt", "\n");
        let mut direction: HashMap<string::String, string::String> = HashMap::new();
        for input in inputs {
            let combination: vec::Vec<string::String> =
                    input.split(")").map(|s| s.to_string()).collect();
            direction.insert(combination[1].clone(), combination[0].clone());
        }
        let mut count = 0;
        let mut orbits: HashMap<string::String, i32> = HashMap::new();
        for (k, _) in &direction {
            count += compute_orbits(&direction, &mut orbits, &k);
        }
        return count;
    }

    fn compute_orbits(direction: &HashMap<string::String, string::String>,
                      orbits: &mut HashMap<string::String, i32>, cur: &str) -> i32 {
        if orbits.contains_key(cur) {
            return *orbits.get(cur).unwrap();
        }
        if !direction.contains_key(cur) {
            return 0;
        }
        let computed_orbits =
                compute_orbits(direction, orbits, direction.get(cur).unwrap()) + 1;
        orbits.insert(cur.to_string(), computed_orbits);
        return computed_orbits;
    }
}

pub mod puzzle_2 {
    use std::collections::HashMap;
    use std::string;
    use std::vec;

    pub fn compute() -> i32 {
        let inputs = crate::get_input("day-6.txt", "\n");
        let mut direction: HashMap<string::String, string::String> = HashMap::new();
        for input in inputs {
            let combination: vec::Vec<string::String> =
                    input.split(")").map(|s| s.to_string()).collect();
            direction.insert(combination[1].clone(), combination[0].clone());
        }
        let mut trace: HashMap<string::String, i32> = HashMap::new();
        let mut search = "SAN".to_string();
        let mut count = 0;
        while direction.contains_key(&search) {
            let new_search = direction.get(&search).unwrap().clone().to_string();
            trace.insert(search, count);
            search = new_search;
            count += 1;
        }
        search = "YOU".to_string();
        count = 0;
        while !trace.contains_key(&search) {
            search = direction.get(&search).unwrap().clone().to_string();
            count += 1;
        }
        return *trace.get(&search).unwrap() + count - 2;
    }
}
