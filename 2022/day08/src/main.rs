fn print_it(grid: &Vec<Vec<char>>) -> () {
    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            print!(
                "{}",
                grid.get(y).unwrap().get(x).unwrap().to_digit(10).unwrap()
            );
        }
        println!();
    }
}

fn get_heigth_at(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    grid.get(y).unwrap().get(x).unwrap().to_digit(10).unwrap()
}

#[test]
fn test_get_heigth_at() -> () {
    let input = std::fs::read_to_string("./src/input.test").unwrap();
    let lines = input.lines();
    let mut tree_grid: Vec<Vec<char>> = Vec::new();

    for (_, line) in lines.enumerate().filter(|l| !l.1.is_empty()) {
        tree_grid.push(line.chars().collect::<Vec<_>>())
    }

    assert_eq!(get_heigth_at(&tree_grid, 0, 0), 3);
    assert_eq!(get_heigth_at(&tree_grid, 1, 0), 0);
    assert_eq!(get_heigth_at(&tree_grid, 2, 0), 3);
    assert_eq!(get_heigth_at(&tree_grid, 3, 0), 7);
    assert_eq!(get_heigth_at(&tree_grid, 4, 0), 3);

    assert_eq!(get_heigth_at(&tree_grid, 0, 0), 3);
    assert_eq!(get_heigth_at(&tree_grid, 0, 1), 2);
    assert_eq!(get_heigth_at(&tree_grid, 0, 2), 6);
    assert_eq!(get_heigth_at(&tree_grid, 0, 3), 3);
    assert_eq!(get_heigth_at(&tree_grid, 0, 4), 3);

    assert_eq!(get_heigth_at(&tree_grid, 1, 1), 5);

    assert_eq!(get_heigth_at(&tree_grid, 2, 2), 3);

    assert_eq!(get_heigth_at(&tree_grid, 3, 3), 4);

    assert_eq!(get_heigth_at(&tree_grid, 4, 4), 0);
}

fn get_right_of(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
    return grid.get(y).unwrap().clone().split_off(x + 1);
}

#[test]
fn test_get_right_of() -> () {
    let input = std::fs::read_to_string("./src/input.test").unwrap();
    let lines = input.lines();
    let mut tree_grid: Vec<Vec<char>> = Vec::new();

    for (_, line) in lines.enumerate().filter(|l| !l.1.is_empty()) {
        tree_grid.push(line.chars().collect::<Vec<_>>())
    }

    println!("-----");
    print_it(&tree_grid);
    println!("-----");

    assert_eq!(get_right_of(&tree_grid, 0, 0), ['0', '3', '7', '3'])
}

fn get_left_of(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
    let mut f = grid.get(y).unwrap().clone();
    f.truncate(x);
    f.reverse();
    return f;
}

#[test]
fn test_get_left_of() -> () {
    let input = std::fs::read_to_string("./src/input.test").unwrap();
    let lines = input.lines();
    let mut tree_grid: Vec<Vec<char>> = Vec::new();

    for (_, line) in lines.enumerate().filter(|l| !l.1.is_empty()) {
        tree_grid.push(line.chars().collect::<Vec<_>>())
    }

    println!("-----");
    print_it(&tree_grid);
    println!("-----");

    assert_eq!(get_left_of(&tree_grid, 4, 3), ['4', '5', '3', '3']);
    assert_eq!(get_left_of(&tree_grid, 2, 2), ['5', '6']);
}

fn get_top_of(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();

    for i in (0..y).rev() {
        result.push(*grid.get(i).unwrap().get(x).unwrap());
    }

    return result;
}

#[test]
fn test_get_top_of() -> () {
    let input = std::fs::read_to_string("./src/input.test").unwrap();
    let lines = input.lines();
    let mut tree_grid: Vec<Vec<char>> = Vec::new();

    for (_, line) in lines.enumerate().filter(|l| !l.1.is_empty()) {
        tree_grid.push(line.chars().collect::<Vec<_>>())
    }

    println!("-----");
    print_it(&tree_grid);
    println!("-----");

    assert_eq!(get_top_of(&tree_grid, 3, 1), ['7']);
    assert_eq!(get_top_of(&tree_grid, 3, 4), ['4', '3', '1', '7']);
}

fn get_bottom_of(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();

    for i in y + 1..grid.len() {
        result.push(*grid.get(i).unwrap().get(x).unwrap());
    }

    return result;
}

#[test]
fn test_get_bottom_of() -> () {
    let input = std::fs::read_to_string("./src/input.test").unwrap();
    let lines = input.lines();
    let mut tree_grid: Vec<Vec<char>> = Vec::new();

    for (_, line) in lines.enumerate().filter(|l| !l.1.is_empty()) {
        tree_grid.push(line.chars().collect::<Vec<_>>())
    }

    println!("-----");
    print_it(&tree_grid);
    println!("-----");

    assert_eq!(get_bottom_of(&tree_grid, 3, 1), ['3', '4', '9']);
    assert_eq!(get_bottom_of(&tree_grid, 3, 4), []);
    assert_eq!(get_bottom_of(&tree_grid, 1, 2), ['3', '5']);
}

fn main() {
    // Read inputs
    let input = std::fs::read_to_string("./src/input.prod").unwrap();

    // Data loading
    let lines = input.lines();
    let mut tree_grid: Vec<Vec<char>> = Vec::new();

    for (_, line) in lines.enumerate().filter(|l| !l.1.is_empty()) {
        tree_grid.push(line.chars().collect::<Vec<_>>())
    }

    print_it(&tree_grid);
    println!();

    // Start part one solving
    let grid_width = tree_grid.first().unwrap().len();
    let grid_height = tree_grid.len();
    let mut visible_tree_count: usize = 0 + (grid_width * 2) + ((grid_height - 2) * 2);
    for y in 1..grid_height - 1 {
        for x in 1..grid_width - 1 {
            let current_height = get_heigth_at(&tree_grid, x, y);
            println!("({},{}) -> {}", x, y, current_height);

            // Try to look upward
            let mut tree_upward = get_top_of(&tree_grid, x, y)
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            tree_upward.sort();
            let highest_tree_upward = *tree_upward.last().unwrap();
            // println!(
            //     "Upward trees are {:?}, highest is {}.",
            //     tree_upward, highest_tree_upward
            // );

            if current_height > highest_tree_upward {
                println!("... current position is visible from the top.");
                visible_tree_count += 1;
                continue;
            }

            // if not visible from upward, look right
            let mut tree_right = get_right_of(&tree_grid, x, y)
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            tree_right.sort();
            let highest_tree_right = *tree_right.last().unwrap();
            // println!(
            //     "Right trees are {:?}, highest is {}.",
            //     tree_right, highest_tree_right
            // );

            if current_height > highest_tree_right {
                println!("... current position is visible from the right.");
                visible_tree_count += 1;
                continue;
            }

            // if not visible from right, look bottom
            let mut tree_bottom = get_bottom_of(&tree_grid, x, y)
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            tree_bottom.sort();
            let highest_tree_bottom = *tree_bottom.last().unwrap();
            // println!(
            //     "Bottom trees are {:?}, highest is {}.",
            //     tree_bottom, highest_tree_bottom
            // );

            if current_height > highest_tree_bottom {
                println!("... current position is visible from the bottom.");
                visible_tree_count += 1;
                continue;
            }

            // if not visible from bottom, look left
            let mut tree_left = get_left_of(&tree_grid, x, y)
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            tree_left.sort();
            let highest_tree_left = *tree_left.last().unwrap();

            if current_height > highest_tree_left {
                println!("... current position is visible from the left.");
                visible_tree_count += 1;
                continue;
            }
        }
        println!();
    }
    println!(
        "Part one solution: number of visible tree is {}.",
        visible_tree_count
    );

    // part two
    let mut scenic_scores: Vec<u32> = Vec::new();
    for y in 1..grid_height - 1 {
        for x in 1..grid_width - 1 {
            let current_height = get_heigth_at(&tree_grid, x, y);
            println!("({},{}) -> {}", x, y, current_height);

            let tree_upward = get_top_of(&tree_grid, x, y)
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            let tree_right = get_right_of(&tree_grid, x, y)
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            let tree_bottom = get_bottom_of(&tree_grid, x, y)
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            let tree_left = get_left_of(&tree_grid, x, y)
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            let mut top_viewing_dist: u32 = 0;
            for i in 0..tree_upward.len() {
                top_viewing_dist += 1;
                if tree_upward.get(i).unwrap() >= &current_height {
                    break;
                }
            }

            let mut right_viewing_dist: u32 = 0;
            for i in 0..tree_right.len() {
                right_viewing_dist += 1;
                if tree_right.get(i).unwrap() >= &current_height {
                    break;
                }
            }

            let mut bottom_viewing_dist: u32 = 0;
            for i in 0..tree_bottom.len() {
                bottom_viewing_dist += 1;
                if tree_bottom.get(i).unwrap() >= &current_height {
                    break;
                }
            }

            let mut left_viewing_dist: u32 = 0;
            for i in 0..tree_left.len() {
                left_viewing_dist += 1;
                if tree_left.get(i).unwrap() >= &current_height {
                    break;
                }
            }

            // println!("top viewing dist: {}", top_viewing_dist);
            // println!("right viewing dist: {}", right_viewing_dist);
            // println!("bottom viewing dist: {}", bottom_viewing_dist);
            // println!("left viewing dist: {} ({:?})", left_viewing_dist, tree_left);

            let scenic_score =
                top_viewing_dist * right_viewing_dist * bottom_viewing_dist * left_viewing_dist;
            println!("Scenic score: {}", scenic_score);
            scenic_scores.push(scenic_score);
        }
    }

    scenic_scores.sort();
    println!(
        "Part two solution: highest scenic score is {}.",
        scenic_scores.last().unwrap()
    );
}
