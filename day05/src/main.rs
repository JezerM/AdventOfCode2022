use std::fs;

enum MoveEnv {
    None,
    Move,
    From,
    To,
}

fn reverse_stacks<T>(stacks: &mut Vec<Vec<T>>) {
    for s in stacks {
        s.reverse();
    }
}
fn get_top_stack(stacks: &Vec<Vec<char>>) -> Vec<char> {
    let mut top_stack: Vec<char> = Vec::new();
    for s in stacks {
        let top = s.last().expect("Uh...");
        top_stack.push(top.to_owned());
    }

    return top_stack;
}

fn move_crates(crate_stacks: &mut Vec<Vec<char>>, quantity: usize, from: usize, to: usize) {
    for _ in 0..quantity {
        let from_stack = crate_stacks.get_mut(from).expect("Uhh... check this");
        let el = from_stack.pop().unwrap();
        let to_stack = crate_stacks.get_mut(to).expect("Uhh... check this");
        to_stack.push(el);
    }
}
fn move_crates_at_once(crate_stacks: &mut Vec<Vec<char>>, quantity: usize, from: usize, to: usize) {
    let mut stack: Vec<char> = Vec::new();

    for _ in 0..quantity {
        let from_stack = crate_stacks.get_mut(from).expect("Uhh... check this");
        let el = from_stack.pop().unwrap();
        stack.push(el);
    }
    stack.reverse();

    let to_stack = crate_stacks.get_mut(to).expect("Uhh... check this");
    to_stack.append(&mut stack);
}

fn get_crate_stacks(input: &String) -> Vec<Vec<char>> {
    let mut crate_stacks: Vec<Vec<char>> = Vec::new();
    let mut stack_index = 0;

    for (i, c) in input.char_indices() {
        if (i + 1) % 2 == 0 && (i + 1) % 4 != 0 {
            if c.is_digit(10) {
                reverse_stacks(&mut crate_stacks);
                break;
            }
            let mut curr_stack = crate_stacks.get_mut(stack_index);
            if curr_stack.is_none() {
                crate_stacks.push(Vec::new());
            }
            curr_stack = crate_stacks.get_mut(stack_index);

            if c != ' ' {
                curr_stack.unwrap().push(c);
            }

            stack_index += 1;
            continue;
        }
        if c == '\n' {
            stack_index = 0;
        }
    }
    return crate_stacks;
}

fn do_crate_movements(input: &String, crate_stacks: &mut Vec<Vec<char>>, multiple_movement: bool) {
    let mut word = String::new();
    let mut move_quantity = 0;
    let mut move_from = 0;
    let mut move_to = 0;
    let mut move_env = MoveEnv::None;

    let mut reading_stack = true;
    for c in input.chars() {
        if reading_stack {
            if c.is_digit(10) {
                reading_stack = false;
            }
            continue;
        }
        if !c.is_whitespace() {
            word.push(c);
            continue;
        }

        match (word.parse::<usize>(), &move_env) {
            (Ok(number), MoveEnv::Move) => {
                move_quantity = number;
            }
            (Ok(number), MoveEnv::From) => {
                move_from = number - 1;
            }
            (Ok(number), MoveEnv::To) => {
                move_to = number - 1;
            }
            _ => {}
        }
        move_env = match word.as_str() {
            "move" => MoveEnv::Move,
            "from" => MoveEnv::From,
            "to" => MoveEnv::To,
            _ => MoveEnv::None,
        };
        word.clear();
        if c != '\n' {
            continue;
        }
        if multiple_movement {
            move_crates_at_once(crate_stacks, move_quantity, move_from, move_to);
        } else {
            move_crates(crate_stacks, move_quantity, move_from, move_to);
        }
        word.clear();
    }
}

fn get_top_stack_one(input: &String) -> String {
    let mut crate_stacks = get_crate_stacks(input);
    do_crate_movements(input, &mut crate_stacks, false);

    let top_stack = get_top_stack(&crate_stacks);
    return top_stack.iter().collect();
}
fn get_top_stack_multiple(input: &String) -> String {
    let mut crate_stacks = get_crate_stacks(input);
    do_crate_movements(input, &mut crate_stacks, true);

    let top_stack = get_top_stack(&crate_stacks);
    return top_stack.iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let top_stack = get_top_stack_one(&input_contents);
        assert_eq!(top_stack, "CMZ");
    }
    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let top_stack = get_top_stack_multiple(&input_contents);
        assert_eq!(top_stack, "MCD");
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let top_stack_one = get_top_stack_one(&input_contents);
    let top_stack_multiple = get_top_stack_multiple(&input_contents);

    println!("Top stack with CrateMover 9000 (part 1): {}", top_stack_one);
    println!(
        "Top stack with CrateMover 9001, multiple (part 2): {}",
        top_stack_multiple
    );
}
