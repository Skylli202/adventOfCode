fn main() {
    println!("Hello, world!\nI'm part 2!");
    let input: &str = include_str!("./input.txt");

    let input_result = part2(input);
    println!("result is {input_result}.");
}

fn part2(input: &str) -> String {
    let mut sum = 0;
    let binding = input.to_string();
    let lines = binding.lines();
    for (index, line) in lines.enumerate() {
        println!("Line {index}: {line} ({sum})");

        let mut line_numbers: Vec<char> = vec![];
        for (char_index, character) in line.chars().enumerate() {
            println!("  Char {char_index}: {character} ({:?})", line_numbers);

            if "123456789".contains(character) {
                line_numbers.push(character);
                println!("   {character} is a number. ({:?})", line_numbers);
                continue;
            }

            let written_numbers = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];

            for (index, written_number) in written_numbers.iter().enumerate() {
                if let Some(num) = line.get(char_index..(char_index + written_number.len())) {
                    if num == *written_number {
                        line_numbers.push(char::from_digit(index as u32 + 1, 10).unwrap());
                        continue;
                    }
                }
            }
        }

        let first = line_numbers.first().unwrap();
        let last = line_numbers.last().unwrap();
        println!("first: {first}, last: {last}");

        let collected_number = first.to_string() + &last.to_string();
        match collected_number.parse::<i32>() {
            Ok(number) => {
                sum += number;
            }
            Err(e) => {
                println!("Error parsing: {:?}", e)
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod test {
    use super::part2;

    #[test]
    fn test_example() {
        let input_example: &str = include_str!("./part2_example.txt");
        let result: String = part2(input_example);
        assert_eq!(result, "281");
    }

    #[test]
    fn test_01() {
        let input_example: &str = "2one";
        let result: String = part2(input_example);
        assert_eq!(result, "21");
    }
}
