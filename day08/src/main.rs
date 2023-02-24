use std::fs;

type Grid = Vec<Vec<u32>>;

// Column = x position
// Row = y position
fn get_value(grid: &Grid, column: usize, row: usize) -> Option<u32> {
    let row_d = grid.get(row);
    if row_d.is_none() {
        return None;
    }
    let value_d = row_d.unwrap().get(column);
    return value_d.copied();
}

fn is_edge(grid: &Grid, column: usize, row: usize) -> bool {
    if row == 0 || column == 0 {
        return true;
    }

    let up = get_value(grid, column, row - 1);
    let down = get_value(grid, column, row + 1);
    let left = get_value(grid, column - 1, row);
    let right = get_value(grid, column + 1, row);

    return up.is_none() || down.is_none() || left.is_none() || right.is_none();
}

fn check_if_visible(grid: &Grid, column: usize, row: usize) -> bool {
    if is_edge(grid, column, row) {
        return true;
    }

    let value = get_value(grid, column, row).unwrap();

    let mut left_visible = true;
    for x in 0..column {
        let el = get_value(grid, x, row).unwrap();
        if value <= el {
            left_visible = false;
            break;
        }
    }
    let mut right_visible = true;
    for x in column + 1..grid.len() {
        let el = get_value(grid, x, row).unwrap();
        if value <= el {
            right_visible = false;
            break;
        }
    }

    let mut up_visible = true;
    for y in 0..row {
        let el = get_value(grid, column, y).unwrap();
        if value <= el {
            up_visible = false;
            break;
        }
    }
    let mut down_visible = true;
    for y in row + 1..grid.len() {
        let el = get_value(grid, column, y).unwrap();
        if value <= el {
            down_visible = false;
            break;
        }
    }

    return up_visible || down_visible || left_visible || right_visible;
}

fn get_scenic_score(grid: &Grid, column: usize, row: usize) -> u32 {
    let value = get_value(grid, column, row).unwrap();

    let mut left_trees = 0;
    for x in (0..column).rev() {
        let el = get_value(grid, x, row).unwrap();
        left_trees += 1;
        if el >= value {
            break;
        }
    }
    let mut right_trees = 0;
    for x in column + 1..grid.len() {
        let el = get_value(grid, x, row).unwrap();
        right_trees += 1;
        if el >= value {
            break;
        }
    }

    let mut up_trees = 0;
    for y in (0..row).rev() {
        let el = get_value(grid, column, y).unwrap();
        up_trees += 1;
        if el >= value {
            break;
        }
    }
    let mut down_trees = 0;
    for y in row + 1..grid.len() {
        let el = get_value(grid, column, y).unwrap();
        down_trees += 1;
        if el >= value {
            break;
        }
    }

    return up_trees * down_trees * left_trees * right_trees;
}

fn get_total_visible_trees(grid: &Grid) -> u32 {
    let mut visible_trees = 0;

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if check_if_visible(grid, x, y) {
                visible_trees += 1;
            }
        }
    }

    return visible_trees;
}

fn get_highest_scenic_score(grid: &Grid) -> u32 {
    let mut highest_score = 0;

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            let score = get_scenic_score(grid, x, y);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    return highest_score;
}

fn parse_input(input: &String) -> Grid {
    let mut grid: Grid = Vec::new();

    let lines = input.lines();

    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            let num = c.to_digit(10).unwrap();
            row.push(num);
        }
        grid.push(row);
    }
    return grid;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let grid = parse_input(&input_contents);

        let visible_trees = get_total_visible_trees(&grid);

        assert_eq!(visible_trees, 21);
    }
    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let grid = parse_input(&input_contents);

        let highest_score = get_highest_scenic_score(&grid);

        assert_eq!(highest_score, 8);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let grid = parse_input(&input_contents);

    let visible_trees = get_total_visible_trees(&grid);
    let highest_score = get_highest_scenic_score(&grid);

    println!(
        "Visible trees from outside the grid (part one): {}",
        visible_trees
    );
    println!(
        "Highest scenic score for any tree (part two): {}",
        highest_score
    );
}
