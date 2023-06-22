use std::fs;

#[derive(Debug, Clone)]
enum Operation {
    Sum(usize),
    Multiply(usize),
    SumSelf,
    MultiplySelf,
}
#[derive(Debug, Clone, Copy)]
enum Relief {
    DIVIDE(usize),
    MODULO(usize),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    items_inspected: usize,
    operation: Operation,
    test: usize,
    throw_true: usize,
    throw_false: usize,
}

fn parse_items(string: &str) -> Vec<usize> {
    let mut items = Vec::new();

    string.split(", ").for_each(|v| {
        if let Ok(num) = usize::from_str_radix(v, 10) {
            items.push(num);
        }
    });
    items
}
fn parse_operation(string: &str) -> Option<Operation> {
    let operation_char = string.as_bytes().get(0);
    let value_str_opt = string.get(2..);

    return match (operation_char, value_str_opt) {
        (Some(b'*'), Some("old")) => Some(Operation::MultiplySelf),
        (Some(b'+'), Some("old")) => Some(Operation::SumSelf),
        (Some(b'*'), Some(value)) => {
            if let Some(num) = usize::from_str_radix(value, 10).ok() {
                Some(Operation::Multiply(num))
            } else {
                None
            }
        }
        (Some(b'+'), Some(value)) => {
            if let Some(num) = usize::from_str_radix(value, 10).ok() {
                Some(Operation::Sum(num))
            } else {
                None
            }
        }
        _ => None,
    };
}

fn parse_input(input: &String) -> Vec<Monkey> {
    let mut monkey_list = Vec::new();

    let monkey_sections = input.split("\n\n");

    for section in monkey_sections {
        let mut items: Option<Vec<usize>> = None;
        let mut operation: Option<Operation> = None;
        let mut test: Option<usize> = None;
        let mut throw_true: Option<usize> = None;
        let mut throw_false: Option<usize> = None;

        for line in section.lines() {
            if let Some(items_str) = line.split("Starting items: ").nth(1) {
                items = Some(parse_items(items_str));
            }
            if let Some(operation_str) = line.split("Operation: new = old ").nth(1) {
                operation = parse_operation(operation_str);
            }
            if let Some(divisible_str) = line.split("divisible by ").nth(1) {
                test = usize::from_str_radix(divisible_str, 10).ok();
            }
            if let Some(true_str) = line.split("If true: throw to monkey ").nth(1) {
                throw_true = usize::from_str_radix(true_str, 10).ok();
            }
            if let Some(false_str) = line.split("If false: throw to monkey ").nth(1) {
                throw_false = usize::from_str_radix(false_str, 10).ok();
            }
        }
        if items.is_none()
            || operation.is_none()
            || test.is_none()
            || throw_true.is_none()
            || throw_false.is_none()
        {
            continue;
        }

        let monkey = Monkey {
            items: items.unwrap(),
            items_inspected: 0,
            operation: operation.unwrap(),
            test: test.unwrap(),
            throw_true: throw_true.unwrap(),
            throw_false: throw_false.unwrap(),
        };
        monkey_list.push(monkey);
    }

    monkey_list
}

fn run_monkey_round(monkey: Monkey, monkey_list: &mut Vec<Monkey>, relief: Relief) {
    for item in monkey.items.iter() {
        let mut item_value = *item;
        match monkey.operation {
            Operation::Sum(value) => item_value += value,
            Operation::Multiply(value) => item_value *= value,
            Operation::SumSelf => item_value += item_value,
            Operation::MultiplySelf => item_value *= item_value,
        }
        item_value = match relief {
            Relief::DIVIDE(value) => item_value / value,
            Relief::MODULO(value) => item_value % value,
        };
        let monkey_to_throw = match item_value % monkey.test == 0 {
            true => monkey_list.get_mut(monkey.throw_true).unwrap(),
            false => monkey_list.get_mut(monkey.throw_false).unwrap(),
        };
        monkey_to_throw.items.push(item_value);
    }
}

fn run_monkey_list_rounds(monkey_list: &Vec<Monkey>, rounds: usize, relief: Relief) -> Vec<Monkey> {
    let mut new_monkey_list = monkey_list.clone();
    for _ in 0..rounds {
        for i in 0..monkey_list.len() {
            if let Some(monkey) = new_monkey_list.get_mut(i) {
                run_monkey_round(monkey.clone(), &mut new_monkey_list, relief);
            }
            if let Some(monkey) = new_monkey_list.get_mut(i) {
                monkey.items_inspected += monkey.items.len();
                monkey.items.clear();
            }
        }
    }
    return new_monkey_list;
}

fn get_monkey_business(monkey_list: &Vec<Monkey>) -> usize {
    let mut sorted_list = monkey_list.clone();
    sorted_list.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    sorted_list
        .get(0..2)
        .unwrap()
        .iter()
        .map(|v| v.items_inspected)
        .reduce(|v, k| v * k)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let monkey_list = parse_input(&input_contents);
        let new_monkey_list = run_monkey_list_rounds(&monkey_list, 20, Relief::DIVIDE(3));

        assert_eq!(get_monkey_business(&new_monkey_list), 10605);
    }

    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let monkey_list = parse_input(&input_contents);

        let mut relief: usize = 1;
        monkey_list.iter().for_each(|v| relief *= v.test);
        let new_monkey_list = run_monkey_list_rounds(&monkey_list, 10000, Relief::MODULO(relief));

        assert_eq!(get_monkey_business(&new_monkey_list), 2713310158);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");
    let monkey_list = parse_input(&input_contents);

    let part_one_list = run_monkey_list_rounds(&monkey_list, 20, Relief::DIVIDE(3));

    let mut relief: usize = 1;
    monkey_list.iter().for_each(|v| relief *= v.test);

    let part_two_list = run_monkey_list_rounds(&monkey_list, 10000, Relief::MODULO(relief));

    println!(
        "Level of monkey business (part one): {}",
        get_monkey_business(&part_one_list)
    );
    println!(
        "Level of monkey business (part two): {}",
        get_monkey_business(&part_two_list)
    );
}
