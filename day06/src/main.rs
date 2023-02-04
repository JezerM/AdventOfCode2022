use std::fs;

fn check_repeats(string: &String) -> bool {
    let mut i: usize = 0;
    while i < string.len() {
        let mut j = i + 1;
        while j < string.len() {
            if string.as_bytes()[i] == string.as_bytes()[j] {
                return true;
            }
            j += 1;
        }
        i += 1;
    }
    return false;
}

fn find_marker(string: &String, marker_length: usize) -> u32 {
    let mut tmp_str = String::new();
    for (i, c) in string.char_indices() {
        if tmp_str.len() == marker_length {
            if !check_repeats(&tmp_str) {
                return i.try_into().unwrap();
            }
            tmp_str.remove(0);
        }
        tmp_str.push(c);
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn do_test(marker_length: usize, expected: u32) {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let mut sequence = String::new();
        let mut sum_of_markers = 0;
        for c in input_contents.chars() {
            if c == '\n' {
                let marker = find_marker(&sequence, marker_length);
                sum_of_markers += marker;
                sequence.clear();
                continue;
            }
            sequence.push(c);
        }
        assert_eq!(sum_of_markers, expected);
    }

    #[test]
    fn part_one() {
        do_test(4, 39);
    }
    #[test]
    fn part_two() {
        do_test(14, 120);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let marker = find_marker(&input_contents, 4);
    println!("Marker (one) found at: {}", marker);

    let marker = find_marker(&input_contents, 14);
    println!("Marker (two) found at: {}", marker);
}
