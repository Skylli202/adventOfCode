use std::u32;

fn main() {
    println!("Hello there!\nI'm part1.");
    let input: &str = include_str!("../input.txt");
    let part1_solution = part1(input);
    println!("Solution: {part1_solution}");
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();

    let mut result: u32 = 0;
    lines.for_each(|line| {
        let (_, line) = line.split_once(':').unwrap();
        let (winning_numbers, numbers) = line.split_once('|').unwrap();
        let winning_numbers = winning_numbers
            .trim()
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let numbers = numbers
            .trim()
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|v| !v.is_empty())
            .map(|v| v.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut amount_of_winning_number: u32 = 0;
        for number in numbers.clone() {
            if winning_numbers.contains(&number) {
                amount_of_winning_number += 1;
            }
        }

        let mut line_score: u32 = 0;
        if amount_of_winning_number > 0 {
            line_score = 2u32.pow(amount_of_winning_number - 1);
        }

        // println!(
        //     "{:?} {:?} {amount_of_winning_number} {}",
        //     winning_numbers, numbers, line_score
        // );

        result += line_score;
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input_example: &str = include_str!("../input_example.txt");
        let solution = part1(input_example);
        assert_eq!(solution, 13u32);
    }
}
