use std::io::BufRead;

fn is_section_completely_containing(sec1: (u8, u8), sec2: (u8, u8)) -> bool {
    match (sec1.0 <= sec2.0 && sec1.1 >= sec2.1) || (sec2.0 <= sec1.0 && sec2.1 >= sec1.1) {
        true => return true,
        false => return false,
    }
}

#[test]
fn test_is_section_completely_containing_01() {
    let sec1: (u8, u8) = (2, 4);
    let sec2: (u8, u8) = (6, 8);
    assert_eq!(false, is_section_completely_containing(sec1, sec2))
}

#[test]
fn test_is_section_completely_containing_02() {
    let sec1: (u8, u8) = (2, 3);
    let sec2: (u8, u8) = (4, 5);
    assert_eq!(false, is_section_completely_containing(sec1, sec2))
}

#[test]
fn test_is_section_completely_containing_03() {
    let sec1: (u8, u8) = (5, 7);
    let sec2: (u8, u8) = (7, 9);
    assert_eq!(false, is_section_completely_containing(sec1, sec2))
}
dTlGKXJRJDx4JnlpeUYuU3ZC
#[test]
fn test_is_section_completely_containing_04() {
    let sec1: (u8, u8) = (2, 8);
    let sec2: (u8, u8) = (3, 7);
    assert_eq!(true, is_section_completely_containing(sec1, sec2))
}

#[test]
fn test_is_section_completely_containing_05() {
    let sec1: (u8, u8) = (3, 7);
    let sec2: (u8, u8) = (2, 8);
    assert_eq!(true, is_section_completely_containing(sec1, sec2))
}

fn is_section_overlaping(sec1: (u8, u8), sec2: (u8, u8)) -> bool {
    if sec1.0 <= sec2.1 && sec1.1 >= sec2.0 {
        return true;
    } else {
        return false;
    }
}

#[test]
fn test_is_section_overlaping_01() {
    let sec1: (u8, u8) = (2, 4);
    let sec2: (u8, u8) = (6, 8);
    assert_eq!(false, is_section_overlaping(sec1, sec2))
}

#[test]
fn test_is_section_overlaping_02() {
    let sec1: (u8, u8) = (5, 7);
    let sec2: (u8, u8) = (7, 9);
    assert_eq!(true, is_section_overlaping(sec1, sec2))
}

#[test]
fn test_is_section_overlaping_03() {
    let sec1: (u8, u8) = (2, 3);
    let sec2: (u8, u8) = (4, 5);
    assert_eq!(false, is_section_overlaping(sec1, sec2))
}

#[test]
fn test_is_section_overlaping_04() {
    let sec1: (u8, u8) = (2, 8);
    let sec2: (u8, u8) = (3, 7);
    assert_eq!(true, is_section_overlaping(sec1, sec2))
}

#[test]
fn test_is_section_overlapping_05() {
    let sec1: (u8, u8) = (6, 6);
    let sec2: (u8, u8) = (4, 6);
    assert_eq!(true, is_section_overlaping(sec1, sec2));
}

#[test]
fn test_is_section_overlapping_06() {
    let sec1: (u8, u8) = (2, 6);
    let sec2: (u8, u8) = (4, 8);
    assert_eq!(true, is_section_overlaping(sec1, sec2));
}


fn main() {
    println!("Advent of code ~ Day 03!");
    part_one();
    part_two();
}

fn part_one() {
    let path = "./assets/puzzle-input.txt";
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut contain_count: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let sectors: Vec<&str> = line.split(',').collect();
        let sector1: Vec<_> = sectors[0].split('-').collect();
        let sector2: Vec<_> = sectors[1].split('-').collect();
        let sector1 = (
            sector1[0].parse::<u8>().unwrap(),
            sector1[1].parse::<u8>().unwrap(),
        );
        let sector2 = (
            sector2[0].parse::<u8>().unwrap(),
            sector2[1].parse::<u8>().unwrap(),
        );
        if is_section_completely_containing(sector1, sector2) {
            contain_count += 1;
        }
    }
    println!("complete overlap counted : {}", contain_count);
}

fn part_two() {
    let path = "./assets/puzzle-input.txt";
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut overlap_count: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let sectors: Vec<&str> = line.split(',').collect();
        let sector1: Vec<_> = sectors[0].split('-').collect();
        let sector2: Vec<_> = sectors[1].split('-').collect();
        let sector1 = (
            sector1[0].parse::<u8>().unwrap(),
            sector1[1].parse::<u8>().unwrap(),
        );
        let sector2 = (
            sector2[0].parse::<u8>().unwrap(),
            sector2[1].parse::<u8>().unwrap(),
        );
        if is_section_overlaping(sector1, sector2) {
            overlap_count += 1;
        }
    }
    println!("overlap count : {}", overlap_count);
}
