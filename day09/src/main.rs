use std::{collections::HashSet, fs};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        Knot { x, y }
    }

    fn touches_knot(&self, knot: &Self) -> bool {
        let rel_x = self.x.abs_diff(knot.x);
        let rel_y = self.y.abs_diff(knot.y);

        return rel_x <= 1 && rel_y <= 1;
    }
    fn move_rel(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
    // If both knots are in the same column or row, move self to knot
    // If they are not in the same column or row, move self diagonally to knot
    fn move_closer_to(&mut self, knot: &Self) {
        let rel_x = knot.x - self.x;
        let rel_y = knot.y - self.y;

        if rel_x == 0 && rel_y.abs() > 1 {
            self.y += if rel_y > 0 { 1 } else { -1 }
        } else if rel_y == 0 && rel_x.abs() > 1 {
            self.x += if rel_x > 0 { 1 } else { -1 }
        } else if rel_y.abs() > 1 || rel_x.abs() > 1 {
            self.x += if rel_x > 0 { 1 } else { -1 };
            self.y += if rel_y > 0 { 1 } else { -1 }
        }
    }
}

fn simulate_rope(input: &String) -> HashSet<Knot> {
    return simulate_long_rope(input, 1);
}

fn simulate_long_rope(input: &String, length: u32) -> HashSet<Knot> {
    let mut set = HashSet::new();

    let mut head = Knot::new(0, 0);
    let mut rope = Vec::new();

    if length == 0 {
        panic!("Length must be greater than 0");
    }

    for _ in 0..length {
        rope.push(Knot::new(0, 0));
    }

    for line in input.lines() {
        let mut splitted = line.split_whitespace();
        let direction = splitted.next().unwrap();
        let steps = splitted.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..steps {
            match direction {
                "R" => head.move_rel(1, 0),
                "L" => head.move_rel(-1, 0),
                "U" => head.move_rel(0, 1),
                "D" => head.move_rel(0, -1),
                _ => {}
            }

            let mut rope_head = head;
            for knot in &mut rope {
                if !knot.touches_knot(&rope_head) {
                    knot.move_closer_to(&rope_head);
                }
                rope_head = knot.to_owned();
            }
            let rope_tail = rope.last().unwrap();
            set.insert(rope_tail.to_owned());
        }
    }

    return set;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let set = simulate_rope(&input_contents);

        assert_eq!(set.len(), 13);
    }
    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let set = simulate_long_rope(&input_contents, 9);

        assert_eq!(set.len(), 1);
    }
    #[test]
    fn part_two_secondary() {
        let input_contents = fs::read_to_string("./test2.txt").expect("Expected test file");

        let set = simulate_long_rope(&input_contents, 9);

        assert_eq!(set.len(), 36);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let simulation_1 = simulate_rope(&input_contents);
    let simulation_2 = simulate_long_rope(&input_contents, 9);

    println!(
        "The tail visited this many positions at least once (part one): {}",
        simulation_1.len()
    );
    println!(
        "The tail (9) visited this many positions at least once (part two): {}",
        simulation_2.len()
    );
}
