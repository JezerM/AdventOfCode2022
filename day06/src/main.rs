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

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let mut sequence = String::new();

    for c in input_contents.chars() {
        if c == '\n' {
            let marker = find_marker(&sequence, 4);
            println!("Marker (one) found at: {}", marker);

            let marker = find_marker(&sequence, 14);
            println!("Marker (two) found at: {}", marker);
            sequence.clear();
            continue;
        }
        sequence.push(c);
    }
}
