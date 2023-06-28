use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    fs,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Node {
    value: char,
    x: usize,
    y: usize,
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{value: {}({}) (x: {}, y: {})}}",
            self.value, self.value as u32, self.x, self.y
        )
    }
}

fn pop_min_dist(
    to_visit: &mut BTreeSet<Node>,
    distances: &BTreeMap<Node, u32>,
) -> Option<(Node, u32)> {
    let filtered: BTreeMap<Node, u32> = distances
        .iter()
        .filter_map(|(node, distance)| match to_visit.contains(node) {
            true => Some((node.clone(), *distance)),
            false => None,
        })
        .collect();

    let mut min_value = filtered.first_key_value();
    for (node, distance) in filtered.iter() {
        if let Some(val) = min_value {
            if distance < val.1 {
                min_value = Some((node, distance));
            }
        }
    }

    if let Some(val) = min_value {
        to_visit.remove(val.0);
        return Some((val.0.clone(), val.1.clone()));
    }
    return None;
}
fn get_four_sides(graph: &Vec<Node>, node: &Node) -> Vec<Node> {
    let mut list = Vec::new();

    for u in graph.iter() {
        if list.len() >= 4 {
            break;
        }

        let right_pos = (node.x + 1, node.y);
        let bottom_pos = (node.x, node.y + 1);

        if u.x == right_pos.0 && u.y == right_pos.1 {
            list.push(u.clone());
        }
        if u.x == bottom_pos.0 && u.y == bottom_pos.1 {
            list.push(u.clone());
        }
        if node.x != 0 {
            let left_pos = (node.x - 1, node.y);
            if u.x == left_pos.0 && u.y == left_pos.1 {
                list.push(u.clone());
            }
        }
        if node.y != 0 {
            let top_pos = (node.x, node.y - 1);
            if u.x == top_pos.0 && u.y == top_pos.1 {
                list.push(u.clone());
            }
        }
    }

    return list;
}

fn get_path_steps(previous: &BTreeMap<Node, Node>, initial: &Node) -> u32 {
    let mut steps = 0;
    if previous.contains_key(&initial) {
        let mut opt = previous.get(&initial);
        while let Some(u) = opt {
            //println!("- {}", u);
            opt = previous.get(u);
            steps += 1;
            if steps > 1000000 {
                println!("NOPE, a lot of steps");
                break;
            }
        }
    }
    return steps;
}

fn dijkstra_pathfind(graph: &Vec<Node>, source: &Node, reverse: bool) -> BTreeMap<Node, Node> {
    let mut previous: BTreeMap<Node, Node> = BTreeMap::new();
    let mut distances: BTreeMap<Node, u32> = BTreeMap::new();

    let mut to_visit: BTreeSet<Node> = BTreeSet::new();

    for vertex in graph.iter() {
        distances.insert(vertex.clone(), u32::MAX);
        to_visit.insert(vertex.clone());
    }
    to_visit.insert(source.clone());
    distances.insert(source.clone(), 0);

    while !to_visit.is_empty() {
        let element = pop_min_dist(&mut to_visit, &distances);
        if element.is_none() {
            continue;
        }
        let u = element.unwrap().0;
        //println!("\nElement: {}", u);

        let four_sides = get_four_sides(graph, &u);

        for neighbour in four_sides.iter() {
            let v = neighbour;

            let u_value = u.value as u32;
            let v_value = v.value as u32;
            //println!("\tCurrent: {}, Neighbour: {}", u, v,);

            let range = if reverse {
                u_value - 1..=u32::MAX
            } else {
                0..=u_value + 1
            };
            //println!("\t\tRange: {:?}", range);

            if !range.contains(&v_value) {
                //println!("\t\t\tNOPE not in range");
                continue;
            }

            let u_distance = *distances.get(&u).unwrap();
            let v_distance = *distances.get(&v).unwrap();

            //println!("\t\tDistances:");
            //println!("\t\t\tCurrent: {}, Neighbour: {}", u_distance, v_distance);

            if u_distance == u32::MAX {
                continue;
            }

            let alt = u_distance + 1;
            if alt <= v_distance {
                previous.insert(v.clone(), u.clone());
                distances.insert(v.clone(), alt);
                //println!("\t\t\tINSERTED PREVIOUS: {} - {}", v, u);
            }
        }
    }

    return previous;
}

fn parse_input(input: &String) -> (Vec<Node>, Node, Node) {
    let mut height_map: Vec<Node> = Vec::new();
    let empty_node = Node {
        value: '\0',
        x: 0,
        y: 0,
    };
    let mut source = empty_node.clone();
    let mut end = empty_node.clone();

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.char_indices() {
            let mut node = Node {
                value: char,
                x: j,
                y: i,
            };
            if char == 'S' {
                node.value = 'a';
                source = node.clone();
            } else if char == 'E' {
                node.value = 'z';
                end = node.clone();
            }
            height_map.push(node);
        }
    }

    return (height_map, source, end);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let (height_map, source, end) = parse_input(&input_contents);
        let previous = dijkstra_pathfind(&height_map, &source, false);
        let steps = get_path_steps(&previous, &end);

        assert_eq!(steps, 31);
    }

    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");
        let (height_map, source, end) = parse_input(&input_contents);
        let previous = dijkstra_pathfind(&height_map, &end, true);

        let mut steps = get_path_steps(&previous, &source);
        for key in previous.keys() {
            if key.value != 'a' {
                continue;
            }
            let s = get_path_steps(&previous, key);
            if s < steps {
                steps = s;
            }
        }
        assert_eq!(steps, 29);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let (height_map, source, end) = parse_input(&input_contents);

    let previous_one = dijkstra_pathfind(&height_map, &source, false);
    let steps_one = get_path_steps(&previous_one, &end);

    let previous_two = dijkstra_pathfind(&height_map, &end, true);
    let mut steps_two = get_path_steps(&previous_two, &source);
    for key in previous_two.keys() {
        if key.value != 'a' {
            continue;
        }
        let s = get_path_steps(&previous_two, key);
        if s < steps_two {
            steps_two = s;
        }
    }

    println!("Minimum steps from E to S (part one): {}", steps_one);
    println!(
        "Minimum steps from any 'a' elevation to S (part two): {}",
        steps_two
    );
}
