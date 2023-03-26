use std::io::BufRead;

fn cumul(calories: &Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for calorie in calories {
        sum += calorie.parse::<i32>().unwrap();
    }
    return sum;
}

#[test]
fn cumul_test_01() {
    let mut my_vec: Vec<String> = Vec::new();

    my_vec.push(String::from("100"));
    assert_eq!(cumul(&my_vec), 100);

    my_vec.push(String::from("50"));
    assert_eq!(cumul(&my_vec), 150);
}

fn sum_three_highest(vec: &Vec<i32>) -> i32 {
    let mut local_vec = vec.to_vec();
    local_vec.sort(); //  vec.sort();
    let local_vec = local_vec.to_vec();

    let size: usize = local_vec.len();
    let highest = local_vec.get(size - 1).unwrap();
    let second_highest = local_vec.get(size - 2).unwrap();
    let third_highest = local_vec.get(size - 3).unwrap();

    highest + second_highest + third_highest
}

#[test]
fn sum_thee_highest_test_01() {
    let mut my_vec: Vec<i32> = Vec::new();

    my_vec.push(1);
    my_vec.push(6);
    my_vec.push(4);
    my_vec.push(5);
    my_vec.push(3);
    my_vec.push(2);

    assert_eq!(15, sum_three_highest(&my_vec));

    my_vec.push(8);
    assert_eq!(19, sum_three_highest(&my_vec));
}

fn main() {
    println!("AOC 2022: Day 01\n");

    // let path = "./assets/puzzle-example.txt";
    let path = "./assets/puzzle-input.txt";
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut elfs: Vec<i32> = Vec::new();
    let mut elf_calories: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "".to_string() {
            elfs.push(cumul(&elf_calories));
            elf_calories = Vec::new();
        } else {
            elf_calories.push(line);
        }
    }

    elfs.sort();
    println!("{}", elfs.get(elfs.len() - 1).unwrap());

    println!(
        "Sum of 3 highest calories are {}.",
        sum_three_highest(&elfs)
    );
}
