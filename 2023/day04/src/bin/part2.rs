fn main() {
    println!("Hello there!\nI'm part2.");
    let input: &str = include_str!("../input.txt");
    let part2_solution = part2(input);
    println!("Solution: {:?}", part2_solution);
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();

    let mut cards = vec![1u32; lines.clone().count()];
    lines.for_each(|line| {
        let (card_number, line) = line.split_once(':').unwrap();
        let (_, card_number) = card_number.split_once(' ').unwrap();
        let current_card_number: usize = card_number.trim().parse::<usize>().unwrap();
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

        let current_card_amount = cards[current_card_number - 1] as usize;
        // println!(
        //     "Card {current_card_number}: {}",
        //     cards[current_card_number - 1]
        // );
        for _ in 0..current_card_amount {
            // println!(" I have {amount_of_winning_number} winning numbers.");
            for j in 1..=amount_of_winning_number {
                let i = j as usize - 1;
                let _old_value = cards[current_card_number + i];
                cards[current_card_number + i] += 1;

                // println!(
                //     "  I am adding one Card {} to {old_value}, I now have {} of them.",
                //     current_card_number + i,
                //     cards[current_card_number + i]
                // );
            }
        }
    });

    let mut sum: u32 = 0;
    for (mut _card_number, card_amount) in cards.iter().enumerate() {
        _card_number += 1;
        // println!("Card {card_number}: {card_amount}");
        sum += card_amount;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input_example: &str = include_str!("../input_example.txt");
        let solution = part2(input_example);
        assert_eq!(solution, 30u32);
    }
}
