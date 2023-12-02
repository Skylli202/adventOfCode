fn main() {
    println!("Hello, world!\nI'm part 1!");
    let input: &str = include_str!("./input.txt");

    let input_result = part1(input);
    println!("result is {input_result}.");
}

fn part1(input: &str) -> String {
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
            }
        }
        // sum += character.to_digit(10).unwrap();

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
    use super::part1;

    #[test]
    fn test_example() {
        let input_example: &str = include_str!("./part1_example.txt");
        let result: String = part1(input_example);
        assert_eq!(result, "142");
    }

    #[test]
    fn test_01() {
        let result: String = part1("1a2\n1bda\n54");
        assert_eq!(result, "77".to_string());
    }
}
