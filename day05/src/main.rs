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

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let mut crate_stacks: Vec<Vec<char>> = Vec::new();
    let mut stack_index = 0;

    let mut word = String::new();
    let mut move_quantity = 0;
    let mut move_from = 0;
    let mut move_to = 0;
    let mut move_env = MoveEnv::None;

    let mut reading_stack = true;

    for (i, c) in input_contents.char_indices() {
        if reading_stack {
            if (i + 1) % 2 == 0 && (i + 1) % 4 != 0 {
                if c.is_digit(10) {
                    reading_stack = false;
                    reverse_stacks(&mut crate_stacks);
                    continue;
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
        } else {
            if c.is_whitespace() {
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
                //move_crates(&mut crate_stacks, move_quantity, move_from, move_to);
                move_crates_at_once(&mut crate_stacks, move_quantity, move_from, move_to);
                word.clear();
                continue;
            }
            word.push(c);
        }
    }

    let top_stack: Vec<char> = get_top_stack(&crate_stacks);
    let top: String = top_stack.iter().collect();

    println!("{}", top);
}
