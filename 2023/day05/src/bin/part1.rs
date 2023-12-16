fn main() {
    println!("Hello, there!\nI'm part1!");
}

fn part1(_input: &str) -> i32 {
    todo!()
}

fn extract_seeds_from_input(input: &str) -> Vec<i32> {
    let mut seeds: Vec<i32> = vec![];

    let mut lines = input.lines();
    if let Some(line_1) = lines.next() {
        dbg!(line_1);
        let (_, mut seeds_str) = line_1.split_once(':').unwrap();

        seeds_str = seeds_str.trim();
        seeds_str.split_whitespace().for_each(|seed_str| {
            if let Ok(seed_i32) = seed_str.parse::<i32>() {
                seeds.push(seed_i32)
            }
        });
    }

    seeds
}

fn convert(origin: i32, convertion_map: &[[i32; 3]]) -> i32 {
    let mut target: i32 = origin;
    for i in convertion_map {
        if target >= i[1] && target <= i[1] + i[2] {
            target = i[0] + (origin - i[1]);
        }
    }
    target
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input: &str = include_str!("../input_example.txt");
        let result: i32 = part1(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_extract_seeds_from_input_01() {
        let input: &str = include_str!("../input_example.txt");
        let seeds = extract_seeds_from_input(input);
        assert_eq!(seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_convert_01() {
        let map: Vec<[i32; 3]> = vec![[50, 98, 2], [52, 50, 48]];

        assert_eq!(convert(79i32, &map), 81i32);
        assert_eq!(convert(14i32, &map), 14i32);
        assert_eq!(convert(55i32, &map), 57i32);
        assert_eq!(convert(13i32, &map), 13i32);
    }
}
