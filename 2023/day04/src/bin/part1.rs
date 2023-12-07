fn main() {
    println!("Hello there!\nI'm part1.");
    let input: &str = include_str!("../input.txt");
    let part1_solution = part1(input);
    println!("Solution: {part1_solution}");
}

fn part1(input: &str) -> usize {
    let lines = input.lines();

    lines.for_each(|line| {
        let println!("{line}");
    });

    0usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input_example: &str = include_str!("../input_example.txt");
        let solution = part1(input_example);
        assert_eq!(solution, 13usize);
    }
}
