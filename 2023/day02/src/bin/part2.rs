#[derive(Default, Debug, Clone, Copy)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

struct InputLine {
    sets: Vec<Set>,
}

fn parse_input_line(line: &str) -> InputLine {
    if let Some((_, sets)) = line.split_once(':') {
        let mut result = InputLine { sets: vec![] };

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
}

fn main() {
    println!("Hi, I'm part2!");
    let input = include_str!("../input.txt");
    let part2_result = part2(input);
    println!("Part2 result: {part2_result}");
}

fn part2(input: &str) -> String {
    let mut power_sum: usize = 0;
    let lines = input.lines();
    for line in lines {
        let input_line: InputLine = parse_input_line(line);

        let mut min_red: usize = 0;
        let mut min_green: usize = 0;
        let mut min_blue: usize = 0;

        for set in input_line.sets {
            if set.red > min_red {
                min_red = set.red;
            }

            if set.green > min_green {
                min_green = set.green;
            }

            if set.blue > min_blue {
                min_blue = set.blue;
            }
        }

        let line_power = min_red * min_green * min_blue;
        power_sum += line_power;
    }

    power_sum.to_string()
}
