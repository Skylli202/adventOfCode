fn main() {
    let input = std::fs::read_to_string("src/input.prod").unwrap();

    let mut stack = vec![("/", 0)];
    let mut final_countdown = vec![];

    let mut total = 0;

    let total_space = 70000000;
    let space_to_delete = 30000000;

    for line in input.lines().filter(|l| !l.is_empty()) {
        // if line == "$ cd /" || line == "$ ls" {

        // Ignore the change directory command to root directory and list commands
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        // Detect the change directory commands
        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (name, amount) = stack.pop().unwrap();
                if amount <= 100_000 {
                    total += amount;
                }
                stack.last_mut().unwrap().1 += amount;
                final_countdown.push((name, amount));
            } else {
                stack.push((dir, 0));
            }

            // println!("{} -> {:?}", line, stack);
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        } else if amount == "dir" {
            // ignore
        }

        // println!("{} -> {:?}", line, stack);
    }

    while stack.len() > 0 {
        let (name, amount) = stack.pop().unwrap();
        final_countdown.push((name, amount));

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += amount;
        }
    }

    // println!("final countdown:\n{:?}", final_countdown);

    let part1_result: usize = final_countdown
        .clone()
        .into_iter()
        .filter(|(_, amount)| *amount < 100_000)
        .map(|(_, amount)| amount)
        .sum();

    println!("Solution to part one is {}.", part1_result);

    let root = final_countdown.last().unwrap();
    let free_space = total_space - root.1;
    println!("free space: {:?}", free_space);
    let space_required = space_to_delete - free_space;
    println!("required space: {:?}", space_required);

    let part2_result = final_countdown
        .clone()
        .into_iter()
        .filter(|(_, amount)| *amount > space_required)
        .map(|(_, amount)| amount)
        .min()
        .unwrap();
    println!("Solution to part two is {}.", part2_result);
}
