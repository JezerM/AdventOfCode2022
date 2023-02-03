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

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let mut word = String::new();
    let mut elf1_str = String::new();
    let mut elf2_str = String::new();

    let mut elf1: Range;
    let mut elf2: Range;

    let mut total_full_contains: u32 = 0;
    let mut total_overlaps: u32 = 0;

    for c in input_contents.chars() {
        if c == ',' {
            elf1_str.push_str(word.as_str());
            word.clear();
            continue;
        }
        if c == '\n' {
            elf2_str.push_str(word.as_str());
            word.clear();

            elf1 = Range::from(&elf1_str);
            elf2 = Range::from(&elf2_str);

            //println!("{} - {}", elf1, elf2);

            if Range::mutual_contains(&elf1, &elf2) {
                total_full_contains += 1;
            }
            if Range::mutual_overlap(&elf1, &elf2) {
                total_overlaps += 1;
            }

            elf1_str.clear();
            elf2_str.clear();
            continue;
        }

        word.push(c);
    }

    println!(
        "Total assignment pairs that fully contain the other: {}",
        total_full_contains
    );
    println!("Total assignment pairs overlaps: {}", total_overlaps);
}
