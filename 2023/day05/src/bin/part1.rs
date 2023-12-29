use std::{str::FromStr, u128};

fn main() {
    println!("Hello, there!\nI'm part1!");
    let input: &str = include_str!("../input.txt");
    let result_part1: u128 = part1(input);
    println!("Result part1: {result_part1}");
}

fn part1(input: &str) -> u128 {
    let mut seeds: Vec<u128> = extract_seeds_from_input(input);

    let mut convertion_maps: Vec<Vec<Vec<u128>>> = Vec::new();
    let mut tmp_map: Vec<Vec<u128>> = Vec::new();

    let mut lines = input.lines();
    let _ = lines.next();
    lines
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .enumerate()
        .for_each(|(_, l)| {
            let str = String::from_str(l).unwrap();
            if !str.contains("map") {
                let inputs: Vec<u128> = l.split(' ').map(|e| e.parse::<u128>().unwrap()).collect();
                // println!("[{i}]:{l}\n   {:?}", inputs);
                tmp_map.push(inputs);
            } else if !tmp_map.is_empty() {
                convertion_maps.push(tmp_map.clone());
                tmp_map = Vec::new();
            }
        });

    if !tmp_map.is_empty() {
        convertion_maps.push(tmp_map.clone());
    }

    // dbg!(convertion_maps.clone());
    convertion_maps.iter().enumerate().for_each(|(i, map)| {
        // println!("Map: {:?}", map);

        seeds = seeds.iter().map(|seed| convert(*seed, map)).collect();
        // println!("after convertion: {:?}", seeds);
        if i < convertion_maps.len() - 1 {
            // println!();
        }
    });

    seeds.sort();
    seeds[0]
}

fn extract_seeds_from_input(input: &str) -> Vec<u128> {
    let mut seeds: Vec<u128> = vec![];

    let mut lines = input.lines();
    if let Some(line_1) = lines.next() {
        let (_, mut seeds_str) = line_1.split_once(':').unwrap();

        seeds_str = seeds_str.trim();
        seeds_str.split_whitespace().for_each(|seed_str| {
            if let Ok(seed_u128) = seed_str.parse::<u128>() {
                seeds.push(seed_u128)
            }
        });
    }

    seeds
}

fn convert(origin: u128, convertion_map: &Vec<Vec<u128>>) -> u128 {
    let mut target: u128 = origin;
    // println!("Origin: {origin}");
    for row_map in convertion_map {
        if row_map.len() != 3 {
            panic!("Got a map line that is not 3 numbers. {:?}", row_map);
        }

        let range_length = row_map[2];
        let dest_range_start = row_map[0];
        let _dest_range_end = dest_range_start + range_length - 1;
        let source_range_start = row_map[1];
        let source_range_end = source_range_start + range_length - 1;

        // println!(
        //     "{source_range_start}..{} ({range_length}) --> {dest_range_start}..{}",
        //     source_range_end, dest_range_end
        // );

        if origin >= source_range_start && origin <= source_range_end {
            target = dest_range_start + (origin - source_range_start);
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
        let result: u128 = part1(input);
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
        let map: Vec<Vec<u128>> = vec![vec![50, 98, 2], vec![52, 50, 48]];

        assert_eq!(convert(79u128, &map), 81_u128);
        assert_eq!(convert(14u128, &map), 14_u128);
        assert_eq!(convert(55u128, &map), 57_u128);
        assert_eq!(convert(13u128, &map), 13_u128);
    }

    #[test]
    fn test_convert_02() {
        let map: Vec<Vec<u128>> = vec![vec![0, 15, 37], vec![37, 52, 2], vec![39, 0, 15]];

        assert_eq!(convert(81_u128, &map), 81_u128);
        assert_eq!(convert(14u128, &map), 53_u128);
        assert_eq!(convert(57u128, &map), 57_u128);
        assert_eq!(convert(13u128, &map), 52_u128);
    }

    #[test]
    fn test_convert_03() {
        let map: Vec<Vec<u128>> = vec![
            vec![49, 53, 8],
            vec![0, 11, 42],
            vec![42, 0, 7],
            vec![57, 7, 4],
        ];

        assert_eq!(convert(81u128, &map), 81_u128);
        assert_eq!(convert(53u128, &map), 49_u128);
        assert_eq!(convert(57u128, &map), 53_u128);
        assert_eq!(convert(52u128, &map), 41_u128);
    }

    #[test]
    fn test_convert_04() {
        let map: Vec<Vec<u128>> = vec![vec![88, 18, 7], vec![18, 25, 70]];

        assert_eq!(convert(81u128, &map), 74_u128);
        assert_eq!(convert(49u128, &map), 42_u128);
        assert_eq!(convert(53u128, &map), 46_u128);
        assert_eq!(convert(41u128, &map), 34_u128);
    }
}
