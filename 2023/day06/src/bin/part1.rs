use core::panic;

fn main() {
    let input = include_str!("../input.txt");
    let race_records = parse_input(input);
    let result = part1(race_records);
    println!("The result of part 1 is {}", result);
}

#[derive(Debug, PartialEq)]
struct Race {
    duration: u32,
    record: u32,
}

fn part1(races: Vec<Race>) -> u32 {
    let mut result = 1;
    for race in races {
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

        result *= count;
    }

    result
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();

    let time: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    let distance: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    if time.len() != distance.len() {
        panic!("Time and distance should have the same length. Check puzzle input.");
    }

    let mut time_iter = time.into_iter();
    let _ = time_iter.next();

    let mut races: Vec<Race> = Vec::new();

    for (i, t) in time_iter.enumerate() {
        if let Some(d) = distance.get(i + 1) {
            // dbg!(t, d);
            // println!();

            let t = t.parse::<u32>().unwrap();
            let d = d.parse::<u32>().unwrap();

            let tmp = Race {
                duration: t,
                record: d,
            };

            races.push(tmp);
        } else {
            panic!("Could not get the distance of the race, check input")
        }
    }

    races
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../input_example.txt");
        let race_records = parse_input(input);
        let result = part1(race_records);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_parse_input_01() {
        let input = include_str!("../input.txt");
        let race_records = parse_input(input);

        let race_0 = Race {
            duration: 47,
            record: 207,
        };

        let race_1 = Race {
            duration: 84,
            record: 1394,
        };

        let race_2 = Race {
            duration: 74,
            record: 1209,
        };

        let race_3 = Race {
            duration: 67,
            record: 1014,
        };

        assert_eq!(race_records.len(), 4);
        assert_eq!(race_records[0], race_0);
        assert_eq!(race_records[1], race_1);
        assert_eq!(race_records[2], race_2);
        assert_eq!(race_records[3], race_3);
    }
}
