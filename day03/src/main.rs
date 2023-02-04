use std::fs;

static LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

fn get_letter_priority(letter: char) -> u32 {
    let is_uppercase = letter.is_uppercase();

    let mut priority = LETTERS.find(letter.to_ascii_lowercase()).unwrap_or(0);
    priority += if is_uppercase { LETTERS.len() } else { 0 };
    return u32::try_from(priority).unwrap_or(0) + 1;
}

fn iterate_for_both(compartment1: &mut String, compartment2: String) -> char {
    let c1: char;

    let n1 = compartment1.chars().nth(0);
    if n1.is_none() {
        return '\0';
    }
    c1 = n1.unwrap();
    for c2 in compartment2.chars() {
        if c2 == c1 {
            return c1;
        }
    }

    let com = compartment1.replace(c1, "");
    compartment1.clear();
    compartment1.push_str(com.as_str());

    return iterate_for_both(compartment1, compartment2);
}

fn iterate_for_badges(r1: String, r2: String, r3: String) -> char {
    for c1 in r1.chars() {
        for c2 in r2.chars() {
            for c3 in r3.chars() {
                if c1 == c2 && c2 == c3 {
                    return c1;
                }
            }
        }
    }
    return '\0';
}

fn get_both_contains(rucksack: &str) -> char {
    let splitted = rucksack.split_at(rucksack.len() / 2);

    let mut compartment1 = splitted.0.to_string();
    let compartment2 = splitted.1.to_string();

    let found = iterate_for_both(&mut compartment1, compartment2);

    return found;
}

fn get_badge_contains(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> char {
    let r2 = rucksack2.to_string();
    let r3 = rucksack3.to_string();
    return iterate_for_badges(rucksack1.to_string(), r2, r3);
}

fn get_sum_of_priorities(input: &String) -> u32 {
    let mut rucksack = String::new();
    let mut total_priority: u32 = 0;

    for c in input.chars() {
        if c == '\n' {
            let found = get_both_contains(rucksack.as_str());
            total_priority += get_letter_priority(found);
            rucksack.clear();
            continue;
        }
        rucksack.push(c);
    }

    return total_priority;
}

fn get_group_sum_priorities(input: &String) -> u32 {
    let mut rucksack = String::new();
    let mut total_priority: u32 = 0;

    let mut elf1 = String::new();
    let mut elf2 = String::new();
    let mut elf3 = String::new();

    for c in input.chars() {
        if c == '\n' {
            if elf1.is_empty() {
                elf1.push_str(rucksack.as_str());
            } else if elf2.is_empty() {
                elf2.push_str(rucksack.as_str());
            } else if elf3.is_empty() {
                elf3.push_str(rucksack.as_str());
            }
            if elf1.is_empty() || elf2.is_empty() || elf3.is_empty() {
                rucksack.clear();
                continue;
            }
            let found = get_badge_contains(elf1.as_str(), elf2.as_str(), elf3.as_str());
            total_priority += get_letter_priority(found);
            rucksack.clear();
            elf1.clear();
            elf2.clear();
            elf3.clear();
            continue;
        }

        rucksack.push(c);
    }
    return total_priority;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let sum = get_sum_of_priorities(&input_contents);
        assert_eq!(sum, 157);
    }

    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let sum = get_group_sum_priorities(&input_contents);
        assert_eq!(sum, 70);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let sum_priorities = get_sum_of_priorities(&input_contents);
    let group_sum_priorities = get_group_sum_priorities(&input_contents);

    println!(
        "Rucksack sum of item priorities (part 1): {}",
        sum_priorities
    );
    println!(
        "Rucksack sum of item priorities by group (part 2): {}",
        group_sum_priorities
    );
}
