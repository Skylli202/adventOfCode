fn main() {
    let input = include_str!("../input.txt");
    let race_records = parse_input(input);
    let result = part2(race_records);
    println!("The result of part 2 is {}", result);
}

#[derive(Debug, PartialEq)]
struct Race {
    duration: u128,
    record: u128,
}

fn part2(race: Race) -> u128 {
    let mut count = 0;
    for hold_duration in 1..race.duration {
        let speed = hold_duration;
        let remaining_duration = race.duration - hold_duration;
        let distance_traveled = speed * remaining_duration;
        dbg!(hold_duration, speed, remaining_duration, distance_traveled);
        println!();

        if distance_traveled > race.record {
            count += 1;
        }
    }

    count
}

fn parse_input(input: &str) -> Race {
    let mut lines = input.lines();

    let mut time: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    let mut distance: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    if time.len() != distance.len() {
        panic!("Time and distance should have the same length. Check puzzle input.");
    }

    let time: Vec<_> = time.drain(1..).collect();
    let distance: Vec<_> = distance.drain(1..).collect();
    let time = time.join("");
    let distance = distance.join("");

    Race {
        duration: time.parse().unwrap(),
        record: distance.parse().unwrap(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let race = Race {
            duration: 71530,
            record: 940200,
        };
        let result = part2(race);
        assert_eq!(result, 71503);
    }

    #[test]
    fn test_parse_input_01() {
        let input = include_str!("../input_example.txt");
        let race_record = parse_input(input);

        let race = Race {
            duration: 71530,
            record: 940200,
        };

        assert_eq!(race_record, race);
    }
}
