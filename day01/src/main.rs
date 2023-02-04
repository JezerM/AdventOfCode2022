use std::fs;

fn iterate_input(input: &String, mut f: impl FnMut(u32) -> ()) {
    let mut curr_calories = 0;

    let mut word = String::new();

    for c in input.chars() {
        if c == '\n' {
            if !word.is_empty() {
                curr_calories += word.parse::<u32>().unwrap();
                word.clear();
                continue;
            }
            f(curr_calories);
            curr_calories = 0;
            word.clear();
            continue;
        }
        word.push(c);
    }
    f(curr_calories);
}

fn get_max_calories(input: &String) -> u32 {
    let mut max_calories: u32 = 0;

    iterate_input(input, |curr_calories| {
        if curr_calories >= max_calories {
            max_calories = curr_calories;
        }
    });
    return max_calories;
}

fn get_top_calories(input: &String) -> [u32; 3] {
    let mut top_calories: [u32; 4] = [0; 4];

    iterate_input(input, |curr_calories| {
        top_calories[3] = curr_calories;
        top_calories.sort_by(|a, b| b.cmp(a));
    });
    let mut result = [0; 3];
    for i in 0..3 {
        result[i] = top_calories[i];
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn max_calories() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let max_calories = get_max_calories(&input_contents);
        assert_eq!(max_calories, 24000);
    }
    #[test]
    fn top_calories() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let top_calories = get_top_calories(&input_contents);

        let top_calories_sum: u32 = top_calories.iter().sum();
        assert_eq!(top_calories_sum, 45000);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let max_calories = get_max_calories(&input_contents);
    let top_calories = get_top_calories(&input_contents);

    let top_calories_sum: u32 = top_calories.iter().sum();

    println!("Max calories (part 1): {}", max_calories);
    println!("Top calories sum (part 2): {}", top_calories_sum);
}
