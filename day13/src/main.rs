use std::{cmp::Ordering, fs};

#[derive(Debug)]
enum PacketValue {
    Number(u32),
    NoNumber,
    StartList,
    EndList,
}

fn word_to_packet_number(word: &String) -> Option<PacketValue> {
    let value = word.parse::<u32>();
    if let Ok(value) = value {
        return Some(PacketValue::Number(value));
    }
    return Some(PacketValue::NoNumber);
}

fn get_packet_value(packet: &str, position: &mut usize) -> Option<PacketValue> {
    let mut word = String::new();
    loop {
        let char = packet.chars().nth(*position);
        *position += 1;
        match char {
            Some('[') => {
                if word.len() > 0 {
                    *position -= 1;
                    return word_to_packet_number(&word);
                }
                return Some(PacketValue::StartList);
            }
            Some(']') => {
                if word.len() > 0 {
                    *position -= 1;
                    return word_to_packet_number(&word);
                }
                return Some(PacketValue::EndList);
            }
            Some(',') => {
                return word_to_packet_number(&word);
            }
            Some(value) => {
                word.push(value);
            }
            None => return None,
        }
    }
}

fn compare_pair(left: &str, right: &str) -> Option<bool> {
    let mut left_position = 0;
    let mut right_position = 0;
    loop {
        let left_value = get_packet_value(left, &mut left_position);
        let right_value = get_packet_value(right, &mut right_position);

        if left_value.is_none() && right_value.is_some() {
            return Some(true);
        }
        if left_value.is_some() && right_value.is_none() {
            return Some(false);
        }
        if left_value.is_none() && right_value.is_none() {
            return None;
        }
        let left_value = left_value.unwrap();
        let right_value = right_value.unwrap();

        match (left_value, right_value) {
            // Both sides are numbers
            (PacketValue::Number(left), PacketValue::Number(right)) => {
                if left < right {
                    return Some(true);
                }
                if right < left {
                    return Some(false);
                }
            }
            // Left side is a number, while right is a list
            (PacketValue::Number(val), PacketValue::StartList) => {
                let left_str = format!("[{}]", val);
                let right_str = &right[right_position - 1..];
                let comparison = compare_pair(&left_str, right_str);
                if let Some(value) = comparison {
                    return Some(value);
                }
            }
            // Left side is a list, while right is a number
            (PacketValue::StartList, PacketValue::Number(val)) => {
                let left_str = &left[left_position - 1..];
                let right_str = format!("[{}]", val);
                let comparison = compare_pair(left_str, &right_str);
                if let Some(value) = comparison {
                    return Some(value);
                }
            }
            // Left list ends before right one
            (PacketValue::EndList, value) => {
                if !matches!(value, PacketValue::EndList) {
                    return Some(true);
                }
            }
            // Right list ends before left one
            (value, PacketValue::EndList) => {
                if !matches!(value, PacketValue::EndList) {
                    return Some(false);
                }
            }
            _ => {}
        }
    }
}

fn evaluate_packets(input: &String) -> u32 {
    let split = input.split("\n\n");

    let mut indices: Vec<u32> = Vec::new();

    for (i, packet) in split.enumerate() {
        if let Some((left, right)) = packet.split_once("\n") {
            let comparison = compare_pair(left, right);
            if comparison.is_some_and(|v| v == true) {
                indices.push((i + 1).try_into().unwrap());
            }
        }
    }

    indices.iter().sum()
}

fn decode_key_packets(input: &String) -> u32 {
    let input = input.replace("\n\n", "\n");
    let mut packets: Vec<&str> = input.split("\n").collect();

    const DIVIDER_1: &str = "[[2]]";
    const DIVIDER_2: &str = "[[6]]";
    packets.push(DIVIDER_1);
    packets.push(DIVIDER_2);

    packets.sort_by(|a, b| match compare_pair(a, b) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    let mut first: u32 = 0;
    let mut second: u32 = 0;
    for (i, packet) in packets.iter().enumerate() {
        if *packet == DIVIDER_1 {
            first = i.try_into().unwrap();
        } else if *packet == DIVIDER_2 {
            second = i.try_into().unwrap();
        }
    }

    first * second
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let sum_of_indices = evaluate_packets(&input_contents);
        assert_eq!(sum_of_indices, 13);
    }

    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let decode_key = decode_key_packets(&input_contents);
        assert_eq!(decode_key, 140);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let sum_of_indices = evaluate_packets(&input_contents);
    let decode_key = decode_key_packets(&input_contents);

    println!("Sum of indices (part one): {}", sum_of_indices);
    println!("Decode key (part two): {}", decode_key);
}
