use std::{i32, usize};

fn main() {
    println!("Hi, I'm part1!");
    let input = include_str!("../input.txt");
    let part1_result = part1(input);
    println!("Part1 result is {part1_result}.");
}

fn part1(input: &str) -> String {
    let grid = parse_input(input);
    let numbers = Number::from_char_grid(&grid);
    let mut sum = 0;

    for number in numbers {
        if number.has_symbol_around(&grid) {
            sum += number.get_value_from(&grid);
        }
    }

    sum.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines();
    let mut grid: Vec<Vec<char>> = vec![];

    for (_, line) in lines.enumerate() {
        let mut vec_line: Vec<char> = vec![];
        for (_, char) in line.char_indices() {
            vec_line.push(char);
        }

        grid.push(vec_line);
    }

    grid
}

#[derive(Debug, PartialEq, Eq)]
struct Number {
    y: usize,
    x: usize,
    length: usize,
}

impl Number {
    pub fn get_value_from(&self, grid: &[Vec<char>]) -> usize {
        let chars: Vec<char> = self.get_chars_from(grid);

        let str = chars.iter().collect::<String>();
        match str.parse::<usize>() {
            Ok(value) => value,
            Err(e) => panic!(
                "Could not parse {str}, to usize.\n  Number: {:?}\n  Error: {e}",
                self
            ),
        }
    }

    pub fn get_chars_from(&self, grid: &[Vec<char>]) -> Vec<char> {
        let line = &grid[self.y];
        let mut chars: Vec<char> = Vec::new();
        line[self.x..(self.x + self.length)].clone_into(&mut chars);
        chars
    }

    pub fn has_symbol_around(&self, grid: &[Vec<char>]) -> bool {
        let grid_height = grid.len();
        let grid_width = grid[0].len();

        for (x, y) in self.get_positions() {
            if x >= grid_width || y >= grid_height {
                continue;
            }
            let char: char = grid[y][x];
            if !char.is_ascii_digit() && char != '.' {
                return true;
            }
        }

        false
    }

    pub fn get_positions(&self) -> Vec<(usize, usize)> {
        let mut positions: Vec<(i32, i32)> = Vec::new();
        let x = self.x as i32;
        let y = self.y as i32;

        {
            // upper-left
            positions.push((y - 1, x - 1));

            // left
            positions.push((y, x - 1));

            // down-left
            positions.push((y + 1, x - 1));
        }

        {
            let offset = self.length as i32 - 1;
            // upper-right
            positions.push((y - 1, x + offset + 1));

            // right
            positions.push((y, x + offset + 1));

            // down-right
            positions.push((y + 1, x + offset + 1));
        }

        for i in 0..self.length {
            let offset = i as i32;
            // upper
            positions.push((y - 1, x + offset));

            // down
            positions.push((y + 1, x + offset));
        }

        positions
            .iter()
            .filter(|(x, y)| !x.is_negative() && !y.is_negative())
            .map(|(y, x)| (*x as usize, *y as usize))
            .collect()
    }

    pub fn from_char_grid(grid: &[Vec<char>]) -> Vec<Number> {
        let mut numbers = Vec::new();

        for (y, line) in grid.iter().enumerate() {
            // println!("{y} {}", line.iter().collect::<String>());
            let mut line_iter = line.iter();

            let mut x: usize = 0;
            while let Some(char) = line_iter.next() {
                // println!("({x}, {y}) {char}");
                if char.is_ascii_digit() {
                    let mut number: Number = Number { y, x, length: 0 };
                    let mut digits: Vec<char> = vec![];
                    digits.push(*char);
                    for char in line_iter.by_ref() {
                        x += 1;
                        if char.is_ascii_hexdigit() {
                            digits.push(*char);
                        } else {
                            break;
                        }
                    }

                    let str = digits.iter().collect::<String>();
                    number.length = str.len();
                    numbers.push(number);
                }

                x += 1;
            }
        }

        numbers
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input_example = include_str!("../input_example.txt");
        let example_result = part1(input_example);
        assert_eq!(example_result, "4361");
    }

    #[test]
    fn test_01() {
        let input = "..114..";
        let grid = parse_input(input);
        let numbers = Number::from_char_grid(&grid);
        assert_eq!(
            numbers,
            vec![Number {
                y: 0,
                x: 2,
                length: 3
            }]
        )
    }

    #[test]
    fn test_02() {
        let input = ".....114324324324";
        let numbers = Number::from_char_grid(&parse_input(input));
        assert_eq!(
            numbers,
            vec![Number {
                y: 0,
                x: 5,
                length: 12
            }]
        )
    }

    #[test]
    fn test_03() {
        let input = "114324....";
        let numbers = Number::from_char_grid(&parse_input(input));
        assert_eq!(
            numbers,
            vec![Number {
                y: 0,
                x: 0,
                length: 6
            }]
        )
    }

    #[test]
    fn test_04() {
        let input = "...11...43.";
        let numbers = Number::from_char_grid(&parse_input(input));
        assert_eq!(
            numbers,
            vec![
                Number {
                    y: 0,
                    x: 3,
                    length: 2
                },
                Number {
                    y: 0,
                    x: 8,
                    length: 2
                }
            ]
        )
    }

    #[test]
    fn test_05() {
        let input = ".....+.58.";
        let numbers = Number::from_char_grid(&parse_input(input));
        assert_eq!(
            numbers,
            vec![Number {
                y: 0,
                x: 7,
                length: 2
            }]
        )
    }

    #[test]
    fn test_from_char_grid_example() {
        let input_example = include_str!("../input_example.txt");
        let numbers = Number::from_char_grid(&parse_input(input_example));
        assert_eq!(
            numbers,
            vec![
                Number {
                    y: 0,
                    x: 0,
                    length: 3
                },
                Number {
                    y: 0,
                    x: 5,
                    length: 3
                },
                Number {
                    y: 2,
                    x: 2,
                    length: 2
                },
                Number {
                    y: 2,
                    x: 6,
                    length: 3
                },
                Number {
                    y: 4,
                    x: 0,
                    length: 3
                },
                Number {
                    y: 5,
                    x: 7,
                    length: 2
                },
                Number {
                    y: 6,
                    x: 2,
                    length: 3
                },
                Number {
                    y: 7,
                    x: 6,
                    length: 3
                },
                Number {
                    y: 9,
                    x: 1,
                    length: 3
                },
                Number {
                    y: 9,
                    x: 5,
                    length: 3
                },
            ]
        )
    }

    #[test]
    fn test_get_chars_from_01() {
        let input_example = include_str!("../input_example.txt");
        let grid = parse_input(input_example);
        let numbers = Number::from_char_grid(&grid);

        assert_eq!(numbers[0].get_chars_from(&grid), vec!['4', '6', '7']);
        assert_eq!(numbers[1].get_chars_from(&grid), vec!['1', '1', '4']);
        assert_eq!(numbers[2].get_chars_from(&grid), vec!['3', '5']);
        assert_eq!(numbers[3].get_chars_from(&grid), vec!['6', '3', '3']);
    }

    #[test]
    fn test_get_value_from_01() {
        let input_example = include_str!("../input_example.txt");
        let grid = parse_input(input_example);
        let numbers = Number::from_char_grid(&grid);

        assert_eq!(numbers[0].get_value_from(&grid), 467);
        assert_eq!(numbers[1].get_value_from(&grid), 114);
        assert_eq!(numbers[2].get_value_from(&grid), 35);
        assert_eq!(numbers[3].get_value_from(&grid), 633);
    }

    #[test]
    fn test_get_positions_01() {
        let positions = Number {
            y: 3,
            x: 3,
            length: 1,
        }
        .get_positions();
        println!("{:?}", positions);

        assert_eq!(positions.len(), 8);
        assert!(positions.contains(&(3, 2)));
        assert!(positions.contains(&(4, 2)));
    }

    #[test]
    fn test_get_positions_02() {
        let positions = Number {
            y: 3,
            x: 3,
            length: 2,
        }
        .get_positions();
        println!("{:?}", positions);

        assert_eq!(positions.len(), 6 + 2 * 2);
        assert!(positions.contains(&(3, 2)));
    }

    #[test]
    fn test_get_positions_03() {
        let positions = Number {
            y: 0,
            x: 0,
            length: 1,
        }
        .get_positions();
        println!("{:?}", positions);

        assert_eq!(positions.len(), 3);
        assert_eq!(positions[0], (1, 0));
        assert_eq!(positions[1], (1, 1));
        assert_eq!(positions[2], (0, 1));
    }

    #[test]
    fn test_get_positions_04() {
        let positions = Number {
            y: 0,
            x: 0,
            length: 2,
        }
        .get_positions();
        println!("{:?}", positions);

        assert_eq!(positions.len(), 4);
        assert!(positions.contains(&(0, 1)));
        assert!(positions.contains(&(1, 1)));
        assert!(positions.contains(&(2, 1)));
        assert!(positions.contains(&(2, 0)));
    }
}
