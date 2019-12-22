pub mod puzzle_1 {
    use std::i32;
    use std::string;
    use std::vec;

    pub fn compute() -> i32 {
        let input = determine_input();
        let layers = determine_layers(input);
        let mut fewest_count = i32::MAX;
        let mut computation = 0;
        for n in &layers {
            let mut count_zero = 0;
            let mut count_one = 0;
            let mut count_two = 0;
            for c in n.chars() {
                match c {
                    '0' => count_zero += 1,
                    '1' => count_one += 1,
                    '2' => count_two += 1,
                    _ => {}
                }
            }
            if count_zero < fewest_count {
                fewest_count = count_zero;
                computation = count_one * count_two;
            }
        }
        return computation;
    }

    fn determine_input() -> string::String {
        let inputs = crate::get_input("day-8.txt", " ");
        return inputs[0].clone();
    }

    fn determine_layers(inputs: string::String) -> vec::Vec<string::String> {
        let pixels_per_layer = 25 * 6;
        let layer_count = inputs.chars().count() / pixels_per_layer;
        let mut layers: Vec<string::String> = vec::Vec::new();
        for n in 0..layer_count {
            let start_pixel = n * pixels_per_layer;
            let end_pixel = (n + 1) * pixels_per_layer;
            layers.push(inputs[start_pixel..end_pixel].to_string());
        }
        return layers;
    }
}

pub mod puzzle_2 {
    use std::string;
    use std::vec;

    pub fn compute(display: bool) -> i32 {
        let input = determine_input();
        let layers = determine_layers(input);
        let image = determine_image(layers);
        if display {
            print_image(image);
        }
        return 0;
    }

    fn determine_input() -> string::String {
        let inputs = crate::get_input("day-8.txt", " ");
        return inputs[0].clone();
    }

    fn determine_layers(inputs: string::String) -> vec::Vec<string::String> {
        let pixels_per_layer = 25 * 6;
        let layer_count = inputs.chars().count() / pixels_per_layer;
        let mut layers: Vec<string::String> = vec::Vec::new();
        for n in 0..layer_count {
            let start_pixel = n * pixels_per_layer;
            let end_pixel = (n + 1) * pixels_per_layer;
            layers.push(inputs[start_pixel..end_pixel].to_string());
        }
        return layers;
    }

    fn determine_image(layers: vec::Vec<string::String>) -> vec::Vec<char> {
        let mut image = vec::Vec::new();
        for i in 0..layers[0].chars().count() {
            let mut value = '2';
            let mut index = 0;
            while value == '2' {
                let arr: Vec<char> = layers[index].chars().collect();
                value = arr[i];
                index += 1;
            }
            image.push(value);
        }
        return image;
    }

    fn print_image(image: vec::Vec<char>) {
        let pixels_per_line = 25;
        let line_count = 6;
        for i in 0..line_count {
            let mut message = "".to_string();
            for j in 0..pixels_per_line {
                let letter = if image[i * pixels_per_line + j] == '0' { ' ' } else { '1' };
                message.push(letter);
            }
            println!("{}", message);
        }
    }
}
