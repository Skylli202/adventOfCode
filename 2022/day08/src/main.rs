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

fn main() {
    // Read inputs
    let input = std::fs::read_to_string("./src/input.test").unwrap();

    // Data loading
    let lines = input.lines();
    let mut tree_grid: Vec<Vec<char>> = Vec::new();

    for (_, line) in lines.enumerate().filter(|l| !l.1.is_empty()) {
        tree_grid.push(line.chars().collect::<Vec<_>>())
    }

    print_it(&tree_grid);
    println!();

    // Start part one solving
    let mut visible_tree_count: usize = 0;
    for y in 1..tree_grid.len() - 1 {
        for x in 1..tree_grid.first().unwrap().len() - 1 {
            let current_height = tree_grid
                .get(y)
                .unwrap()
                .get(x)
                .unwrap()
                .to_digit(10)
                .unwrap();
            println!("({},{}) -> {}", x, y, current_height);

            let mut is_visible: bool = true;
            // Try to look upward
            for i in 0..y {
                let tree_in_line = tree_grid
                    .get(i)
                    .unwrap()
                    .get(x)
                    .unwrap()
                    .to_digit(10)
                    .unwrap();

                if tree_in_line >= current_height {
                    is_visible = false;
                    break;
                }

                println!("  ({},{}) -> {}", x, i, tree_in_line);
            }

            if is_visible {
                println!("... current position is visible from the top.");
                visible_tree_count += 1;
                continue;
            } else {
                is_visible = true;
            }

            // if not visible from upward, look right
            for i in x + 1..tree_grid.get(y).unwrap().len() {
                let tree_in_line = tree_grid
                    .get(y)
                    .unwrap()
                    .get(i)
                    .unwrap()
                    .to_digit(10)
                    .unwrap();

                if tree_in_line >= current_height {
                    is_visible = false;
                    break;
                }

                println!("  ({},{}) -> {}", i, y, tree_in_line);
            }

            if is_visible {
                println!("... current position is visible from the right.");
                visible_tree_count += 1;
                continue;
            } else {
                is_visible = true;
            }

            // if not visible from right, look bottom
            for i in y + 1..tree_grid.len() {
                let tree_in_line = tree_grid
                    .get(i)
                    .unwrap()
                    .get(x)
                    .unwrap()
                    .to_digit(10)
                    .unwrap();

                if tree_in_line >= current_height {
                    is_visible = false;
                    break;
                }

                println!("  ({},{}) -> {}", x, i, tree_in_line);
            }

            if is_visible {
                println!("... current position is visible from the bottom.");
                visible_tree_count += 1;
                continue;
            } else {
                is_visible = true;
            }

            // if not visible from bottom, look left
        }
        println!();
    }
    println!(
        "Part one solution: number of visible tree is {}.",
        visible_tree_count
    );
}
