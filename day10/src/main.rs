use std::fs;

fn iterate_input(input: &String, mut f: impl FnMut(&mut i32, i32)) {
    let mut cycle = 0;
    let mut register = 1;

    let lines = input.lines();
    for line in lines {
        let mut splitted = line.split_whitespace();
        let call = splitted.next().unwrap();
        let word = splitted.next();

        let effort = match call {
            "noop" => 1,
            "addx" => 2,
            _ => 0,
        };

        for _ in 0..effort {
            f(&mut cycle, register);
        }
        if effort == 2 {
            let word = word.unwrap();
            let num: i32 = word.parse().unwrap();
            register += num;
        }
    }
}

fn get_sum_of_signal_strengths(input: &String) -> i32 {
    let mut total_signal = 0;

    iterate_input(input, |cycle, register| {
        *cycle += 1;
        let n = if *cycle > 20 { *cycle + 20 } else { *cycle };

        if n == 20 || n % 40 == 0 {
            let signal = *cycle * register;
            total_signal += signal;
        }
    });

    return total_signal;
}

fn draw_image(input: &String) -> String {
    let mut image = String::new();

    iterate_input(input, |cycle, register| {
        let n = *cycle % 40;
        if n == 0 {
            image.push('\n');
        }
        if n == register || n == register - 1 || n == register + 1 {
            image.push('#');
        } else {
            image.push('.');
        }
        *cycle += 1;
    });

    return image.trim().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let signal_strength = get_sum_of_signal_strengths(&input_contents);

        assert_eq!(signal_strength, 13140);
    }

    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let mut test_image = fs::read_to_string("./test_image.txt").expect("Expected image");
        test_image = test_image.trim_end().to_string();

        let image = draw_image(&input_contents);

        assert_eq!(test_image, image);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let signal_strength = get_sum_of_signal_strengths(&input_contents);
    let image = draw_image(&input_contents);

    println!(
        "The sum of signal strengths (part one): {}",
        signal_strength
    );

    println!("Drawed image (part two):");
    println!("{image}");
}
