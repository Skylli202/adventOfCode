struct GameConfig {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    println!("Hi, I'm part1!");
    let input = include_str!("../input.txt");
    let part1_result = part1(input);
    println!("Part1 result: {part1_result}");
}

fn part1(input: &str) -> String {
    const GAME_CONFIG: GameConfig = GameConfig {
        red: 12,
        green: 13,
        blue: 14,
    };

    let lines = input.lines();
    let mut ids_sums: usize = 0;

    for line in lines {
        let input_line: InputLine = parse_input_line(line);

        let mut tmp: bool = true;
        for set in input_line.sets {
            if !set.is_valid(GAME_CONFIG) {
                tmp = false;
                break;
            }
        }

        if tmp {
            ids_sums += input_line.id;
        }
    }

    ids_sums.to_string()
}

#[derive(Default, Debug, Clone, Copy)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    pub fn is_valid(&self, game_config: GameConfig) -> bool {
        if self.red > game_config.red {
            return false;
        }

        if self.green > game_config.green {
            return false;
        }

        if self.blue > game_config.blue {
            return false;
        }

        true
    }
}

struct InputLine {
    id: usize,
    sets: Vec<Set>,
}

fn parse_input_line(line: &str) -> InputLine {
    if let Some((part1, sets)) = line.split_once(':') {
        if let Some((_, id)) = part1.split_once(' ') {
            let mut result = InputLine {
                id: id.parse::<usize>().unwrap(),
                sets: vec![],
            };

            for set in sets.split(';') {
                let mut new_draw: Set = Set::default();

                for mut draw in set.split(',') {
                    draw = draw.trim();

                    if let Some((amt, color)) = draw.split_once(' ') {
                        if let Ok(qty) = amt.parse::<usize>() {
                            match color {
                                "red" => new_draw.red = qty,
                                "green" => new_draw.green = qty,
                                "blue" => new_draw.blue = qty,
                                _ => panic!("Unexpected color: {color}"),
                            }
                        } else {
                            panic!("Invalid line: {line}")
                        }
                    } else {
                        panic!("Invalid line: {line}")
                    }
                }

                result.sets.push(new_draw);
            }

            result
        } else {
            panic!("Invalid line: {line}")
        }
    } else {
        panic!("Invalid line: {line}")
    }
}

#[cfg(test)]
mod test {
    use super::{parse_input_line, part1};

    #[test]
    fn test_example() {
        let example_input: &str = include_str!("../example_input.txt");
        let result = part1(example_input);
        assert_eq!(result, "8");
    }

    #[test]
    fn test_01() {
        let line = "Game 1: 3 green, 2 red, 5 blue";
        let parsed_line = parse_input_line(line);
        assert_eq!(parsed_line.id, 1usize);
        assert_eq!(parsed_line.sets.len(), 1);
        assert_eq!(parsed_line.sets[0].red, 2);
        assert_eq!(parsed_line.sets[0].green, 3);
        assert_eq!(parsed_line.sets[0].blue, 5);
    }

    #[test]
    fn test_02() {
        let line = "Game 1: 3 green, 2 red, 5 blue; 12 blue";
        let parsed_line = parse_input_line(line);
        assert_eq!(parsed_line.id, 1usize);
        assert_eq!(parsed_line.sets.len(), 2);
        assert_eq!(parsed_line.sets[0].red, 2);
        assert_eq!(parsed_line.sets[0].green, 3);
        assert_eq!(parsed_line.sets[0].blue, 5);
        assert_eq!(parsed_line.sets[1].red, 0);
        assert_eq!(parsed_line.sets[1].green, 0);
        assert_eq!(parsed_line.sets[1].blue, 12);
    }
}

