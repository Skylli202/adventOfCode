use std::io::BufRead;

fn find_common_items(first_rucksack: &str, second_rucksack: &str) -> Vec<char> {
    let mut common_items: Vec<char> = Vec::new();
    for char in first_rucksack.chars() {
        if let Some(position) = second_rucksack.find(char) {
            if let Some(char) = second_rucksack.chars().nth(position) {
                if !common_items.contains(&char) {
                    common_items.push(char);
                }
            }
        }
    }
    return common_items;
}

#[test]
fn test_find_common_items_01() {
    let first_rucksack: &str = "vJrwpWtwJgWr";
    let second_rucksack: &str = "hcsFMMfFFhFp";

    assert_eq!(
        vec!['p'],
        find_common_items(first_rucksack, second_rucksack)
    )
}

#[test]
fn test_find_common_items_02() {
    let first_rucksack: &str = "jqHRNqRjqzjGDLGL";
    let second_rucksack: &str = "rsFMfFZSrLrFZsSL";

    assert_eq!(
        vec!['L'],
        find_common_items(first_rucksack, second_rucksack)
    )
}

#[test]
fn test_find_common_items_03() {
    let first_rucksack: &str = "PmmdzqPrV";
    let second_rucksack: &str = "vPwwTWBwg";

    assert_eq!(
        vec!['P'],
        find_common_items(first_rucksack, second_rucksack)
    )
}

#[test]
fn test_find_common_items_04() {
    let first_rucksack: &str = "wMqvLMZHhHMvwLH";
    let second_rucksack: &str = "jbvcjnnSBnvTQFn";

    assert_eq!(
        vec!['v'],
        find_common_items(first_rucksack, second_rucksack)
    )
}

#[test]
fn test_find_common_items_05() {
    let first_rucksack: &str = "ttgJtRGJ";
    let second_rucksack: &str = "QctTZtZT";

    assert_eq!(
        vec!['t'],
        find_common_items(first_rucksack, second_rucksack)
    )
}

#[test]
fn test_find_common_items_06() {
    let first_rucksack: &str = "CrZsJsPPZsGz";
    let second_rucksack: &str = "wwsLwLmpwMDw";

    assert_eq!(
        vec!['s'],
        find_common_items(first_rucksack, second_rucksack)
    )
}

fn get_proirity_of_item(item: char) -> u32 {
    let unicode: u32 = item as u32;

    if unicode >= 97 && unicode <= 122 {
        return unicode - 96;
    }

    if unicode >= 65 && unicode <= 90 {
        return unicode - 64 + 26;
    }

    return 0;
}

#[test]
fn test_get_proirity_of_item_01() {
    assert_eq!(1, get_proirity_of_item('a'));
    assert_eq!(26, get_proirity_of_item('z'));
    assert_eq!(27, get_proirity_of_item('A'));
    assert_eq!(52, get_proirity_of_item('Z'));
}

fn main() {
    println!("Advent of code ~ Day 03!");
    part_one();
    part_two();
}

fn part_one() {
    println!("Part 1!");
    // let path = "./assets/puzzle-example.txt";
    let path = "./assets/puzzle-input.txt";
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut sum: u32 = 0;
    for line in reader.lines() {
        let line: String = line.unwrap();
        let size: usize = line.len();
        let (first_rucksack, second_rucksack): (&str, &str) = line.split_at(size / 2);
        assert_eq!(first_rucksack.len(), second_rucksack.len());
        let common_items: Vec<char> = find_common_items(first_rucksack, second_rucksack);
        for item in common_items {
            sum = sum + get_proirity_of_item(item)
        }
    }

    println!("{}", sum)
}

fn part_two() {
    println!("Part 2!");
    // let path = "./assets/puzzle-example.txt";
    let path = "./assets/puzzle-input.txt";
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut line_count: u8 = 0;
    let mut lines: Vec<String> = Vec::new();
    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        line_count += 1;
        lines.push(line);
        if line_count == 3 {
            let common_items_1_2: Vec<char> =
                find_common_items(lines[0].as_str(), lines[1].as_str());
            let common_items_1_3: Vec<char> =
                find_common_items(lines[0].as_str(), lines[2].as_str());

            let common_items: Vec<char> = find_common_items(
                common_items_1_2.iter().collect::<String>().as_str(),
                common_items_1_3.iter().collect::<String>().as_str(),
            );

            for item in common_items {
                sum += get_proirity_of_item(item)
            }

            line_count = 0;
            lines.clear();
        }
    }
    println!("{}", sum)
}
