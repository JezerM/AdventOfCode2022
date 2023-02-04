use std::fmt;
use std::fs;

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn contains(&self, val: &Self) -> bool {
        return self.min <= val.min && self.max >= val.max;
    }
    fn mutual_contains(range1: &Self, range2: &Self) -> bool {
        return range1.contains(range2) || range2.contains(range1);
    }
    fn overlap(&self, val: &Self) -> bool {
        return self.min <= val.max && self.max >= val.min;
    }
    fn mutual_overlap(range1: &Self, range2: &Self) -> bool {
        return range1.overlap(range2) || range2.overlap(range1);
    }
}

impl From<&str> for Range {
    fn from(string: &str) -> Self {
        let min: u32;
        let max: u32;

        let mut splitted = string.split('-');

        let min_str = splitted.next().expect("Expected a string");
        let max_str = splitted.next().expect("Expected a string");

        min = min_str.parse::<u32>().expect("Expected a number");
        max = max_str.parse::<u32>().expect("Expected a number");

        return Range { min, max };
    }
}

impl From<&String> for Range {
    fn from(string: &String) -> Self {
        return Self::from(string.as_str());
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Range {{ {}, {} }}", self.min, self.max)
    }
}

fn iterate_input(input: &String, mut f: impl FnMut(Range, Range) -> ()) {
    let mut elf1_str = String::new();
    let mut elf2_str = String::new();

    let mut word = String::new();

    for c in input.chars() {
        if c == ',' {
            elf1_str.push_str(word.as_str());
            word.clear();
            continue;
        }
        if c == '\n' {
            elf2_str.push_str(word.as_str());
            word.clear();

            let elf1 = Range::from(&elf1_str);
            let elf2 = Range::from(&elf2_str);

            f(elf1, elf2);

            elf1_str.clear();
            elf2_str.clear();
            continue;
        }

        word.push(c);
    }
}

fn get_total_contains(input: &String) -> u32 {
    let mut total_full_contains: u32 = 0;

    iterate_input(input, |elf1, elf2| {
        if Range::mutual_contains(&elf1, &elf2) {
            total_full_contains += 1;
        }
    });

    return total_full_contains;
}
fn get_total_overlaps(input: &String) -> u32 {
    let mut total_overlaps: u32 = 0;

    iterate_input(input, |elf1, elf2| {
        if Range::mutual_overlap(&elf1, &elf2) {
            total_overlaps += 1;
        }
    });

    return total_overlaps;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let total_full_contains: u32 = get_total_contains(&input_contents);
        assert_eq!(total_full_contains, 2);
    }
    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let total_overlaps: u32 = get_total_overlaps(&input_contents);
        assert_eq!(total_overlaps, 4);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let total_full_contains: u32 = get_total_contains(&input_contents);
    let total_overlaps: u32 = get_total_overlaps(&input_contents);

    println!(
        "Total assignment pairs that fully contain the other (part 1): {}",
        total_full_contains
    );
    println!(
        "Total assignment pairs overlaps (part 2): {}",
        total_overlaps
    );
}
