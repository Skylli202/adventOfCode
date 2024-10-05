use core::panic;
use std::str::FromStr;

fn main() {
    println!("Hello, there!\nI'm part2!");
    let input: &str = include_str!("../input.txt");
    let result_part2: u128 = part2(input);
    println!("Result: {result_part2}");
}

fn part2(input: &str) -> u128 {
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

    let mut full_seeds: Vec<u128> = Vec::new();

    if seeds.len() % 2 != 0 {
        panic!("Seeds suppose to go by two but parsed an uneven amount of seeds")
    }

    let mut i = 0;
    while i < seeds.len() {
        let seed_start = seeds.get(i).unwrap();
        i += 1;
        let seed_range = seeds.get(i).unwrap();

        // println!(
        //     "[{}] seed_start: {}, range: {} / {:?}",
        //     i - 1,
        //     seed_start,
        //     seed_range,
        //     seeds
        // );

        for j in 0..*seed_range {
            full_seeds.push(seed_start + j);
        }

        i += 1;
    }
    println!("Number of seed: {}", full_seeds.len());

    full_seeds
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
    use std::u128;

    use super::*;

    #[test]
    fn test_example() {
        let input: &str = include_str!("../input_example.txt");
        let result: u128 = part2(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_extract_seeds_from_input_01() {
        let input: &str = include_str!("../input_example.txt");
        let seeds = extract_seeds_from_input(input);

        let mut arr_1: [u128; 14] = [0; 14];
        let offset_1 = 79_u128;
        for i in 0..14 {
            arr_1[i] = offset_1 + u128::try_from(i).unwrap();
        }

        let mut arr_2: [u128; 13] = [0; 13];
        let offset_2 = 55_u128;
        for i in 0..13 {
            arr_2[i] = offset_2 + u128::try_from(i).unwrap();
        }

        let mut vec: Vec<u128> = Vec::with_capacity(27);
        vec.extend_from_slice(&arr_1);
        vec.extend_from_slice(&arr_2);
        println!("vec: {:?}", vec);
        assert_eq!(seeds, vec);
    }

    #[test]
    fn test_convert_01() {
        let map: Vec<Vec<u128>> = vec![vec![50, 98, 2], vec![52, 50, 48]];

        assert_eq!(convert(79u128, &map), 81_u128);
        assert_eq!(convert(14u128, &map), 14_u128);
        assert_eq!(convert(55u128, &map), 57_u128);
        assert_eq!(convert(13u128, &map), 13_u128);
    }
}
