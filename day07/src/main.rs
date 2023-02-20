use std::fs;
use std::{cell::RefCell, collections::VecDeque, path::PathBuf, rc::Rc};

type Ref<T> = Rc<RefCell<T>>;

#[derive(PartialEq, Debug)]
struct Node {
    name: String,
    size: u32,
    is_directory: bool,
    path: PathBuf,
    parent: Option<Ref<Node>>,
    children: Vec<Ref<Node>>,
}

trait NodeTrait {
    fn add(&self, node: &Self);
    fn get_parent(&self) -> Option<Self>
    where
        Self: Sized;
    fn get_children(&self) -> Vec<Self>
    where
        Self: Sized;
    fn get_name(&self) -> String;
    fn get_path(&self) -> PathBuf;
    fn get_path_str(&self) -> String;
}

impl NodeTrait for Ref<Node> {
    fn add(&self, node: &Self) {
        let mut self_mut = self.borrow_mut();
        if self_mut.children.contains(node) {
            return;
        }
        self_mut.children.push(node.to_owned());
        let mut path = self_mut.path.clone();
        path.push(node.borrow().name.clone());
        node.borrow_mut().path = path;
        node.borrow_mut().parent = Some(self.to_owned());
    }

    fn get_parent(&self) -> Option<Self>
    where
        Self: Sized,
    {
        return self.borrow().parent.clone();
    }
    fn get_children(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        return self.borrow().children.clone();
    }
    fn get_name(&self) -> String {
        return self.borrow().name.clone();
    }
    fn get_path(&self) -> PathBuf {
        return self.borrow().path.clone();
    }
    fn get_path_str(&self) -> String {
        return self.borrow().path.to_str().unwrap_or("").to_string();
    }
}

impl Node {
    fn new(name: String, size: u32, is_directory: bool) -> Ref<Self> {
        let value = Node {
            name,
            size,
            path: PathBuf::from("/"),
            is_directory,
            parent: None,
            children: Vec::new(),
        };
        return Rc::new(RefCell::new(value));
    }
    fn parse_from(input: String) -> Option<Ref<Self>> {
        let mut splitted = input.split_whitespace();

        let size = splitted.next().unwrap();
        let name = splitted.next().unwrap();

        if size == "dir" {
            Some(Node::new(name.to_string(), 0, true))
        } else {
            let size: u32 = match size.parse() {
                Ok(num) => num,
                Err(_) => return None,
            };
            Some(Node::new(name.to_string(), size, false))
        }
    }
}

// Iterate all the nodes using the BFS algorithm
fn bfs_iterate(root: &Ref<Node>, mut f: impl FnMut(&Ref<Node>)) {
    let mut to_visit: VecDeque<Ref<Node>> = VecDeque::new();
    let mut visited: Vec<Ref<Node>> = Vec::new();

    to_visit.push_back(root.clone());

    while !to_visit.is_empty() {
        let v = to_visit.pop_front();
        if v.is_none() {
            return;
        }
        let element = v.unwrap();
        let children = element.get_children();
        for child in children.iter() {
            if !visited.contains(child) {
                visited.push(child.clone());

                to_visit.push_back(child.clone());

                f(child);
            }
        }
    }
}

// Get size of directory
fn get_size(node: &Ref<Node>) -> u64 {
    let mut size: u64 = 0;
    bfs_iterate(node, |child| {
        size += Into::<u64>::into(child.borrow().size);
    });
    return size;
}

// Get concurrent size of directory
// - /a
//    - /e
//      - file (30)
//    - file (20)
// If /a/e sizes 30, then /a sizes 50. So, get_concurrent_size (/a) returns 80
fn get_concurrent_size(node: &Ref<Node>, mut size: u64) -> u64 {
    let n = node.borrow();
    if !n.is_directory {
        return n.size.into();
    }

    for child in node.get_children() {
        size += get_concurrent_size(&child, size);
    }
    return size;
}

fn get_total_size(root: &Ref<Node>, top_size: u32) -> u64 {
    let mut total_size = 0;

    bfs_iterate(root, |child| {
        if child.borrow().is_directory {
            let dir_size = get_concurrent_size(&child, 0);
            if dir_size < top_size.into() {
                total_size += dir_size;
            }
        }
    });

    return total_size;
}

fn find_smallest_node_size_to_delete(
    root: &Ref<Node>,
    total_disk_size: u64,
    required_size: u64,
) -> u64 {
    let total_used_size: u64 = get_size(root);
    let unused_size: u64 = total_disk_size - total_used_size;

    let mut smallest_node_size = total_used_size;

    bfs_iterate(root, |child| {
        if child.borrow().is_directory {
            let dir_size = get_size(&child);

            if unused_size + dir_size >= required_size && dir_size < smallest_node_size {
                smallest_node_size = dir_size;
            }
        }
    });

    return smallest_node_size;
}

// Get relative node to current
fn get_relative_node_at<'a>(
    root: &Ref<Node>,
    current: &Ref<Node>,
    path: &String,
) -> Option<Ref<Node>> {
    if path == "/" {
        return Some(root.clone());
    }

    let mut target = current.get_path();
    if path == ".." {
        return current.get_parent();
    } else {
        target.push(path);
    }

    let children = current.get_children();
    for child in children {
        if child.borrow().path == target {
            return Some(child);
        }
    }
    return None;
}

// Parse the input file to create all the nodes and append them to their parents
fn parse_input(input: &String, root: &Ref<Node>) {
    let lines = input.lines();

    let mut current: Ref<Node> = root.clone();

    for line in lines {
        if line.starts_with('$') {
            let mut spaces = line.split_whitespace();
            spaces.next();
            let command = spaces.next().unwrap().to_string();
            if command == "cd" {
                let arg = spaces.next().unwrap().to_string();
                current = get_relative_node_at(&root, &current, &arg).unwrap();
            }
            continue;
        }
        let node = Node::parse_from(line.to_string());
        if let Some(value) = node {
            current.add(&value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let root = Node::new(String::from("/"), 0, true);
        parse_input(&input_contents, &root);

        let size = get_total_size(&root, 100000);
        println!("Size: {}", size);

        assert_eq!(size, 95437);
    }
    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let root = Node::new(String::from("/"), 0, true);
        parse_input(&input_contents, &root);

        let size = find_smallest_node_size_to_delete(&root, 70000000, 30000000);
        println!("Size: {}", size);

        assert_eq!(size, 24933642);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let root = Node::new(String::from("/"), 0, true);
    parse_input(&input_contents, &root);

    let size = get_total_size(&root, 100000);
    println!(
        "Total size of directories of at most 100000 (part 1): {}",
        size
    );

    let size = find_smallest_node_size_to_delete(&root, 70000000, 30000000);
    println!(
        "Total size of smallest directory to delete (part 2): {}",
        size
    );
}
