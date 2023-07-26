fn add_if_not_in(v: &mut Vec<(f32, f32)>, p: (f32, f32)) -> () {
    match v.iter().position(|&e| e == p) {
        None => v.push(p),
        Some(_) => (),
    }
}

#[test]
fn test_add_if_not_in() -> () {
    let mut v1: Vec<(f32, f32)> = vec![];
    let mut v2: Vec<(f32, f32)> = vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)];

    add_if_not_in(&mut v1, (0.0, 2.0));
    assert_eq!(v1.len(), 1);

    add_if_not_in(&mut v2, (1.0, 0.0));
    assert_eq!(v2.len(), 3);
}

fn main() {
    let mut head_position = (0.0, 0.0);
    let mut tail_position = (0.0, 0.0);

    let input: String = std::fs::read_to_string("./src/input.prod").unwrap();
    let instructions = input.lines();

    // let mut tail_positions: HashMap<isize, (f32, f32)> = HashMap::new();
    let mut tail_positions: Vec<(f32, f32)> = vec![(0.0, 0.0)];
    // let mut instruction_count: u32 = 0;

    for instruction in instructions {
        // instruction_count += 1;
        // if instruction_count >= 10 {
        //     break;
        // }
        let (direction, distance) = instruction.split_once(" ").unwrap();
        println!("Dir:{}, dist:{}", direction, distance);
        let distance = distance.parse::<isize>();
        let distance = match distance {
            Ok(d) => d,
            Err(err) => panic!(
                "Could not parse the following instruction {instruction}.\n{}",
                err
            ),
        };
        println!("{distance}");

        for _ in 0..distance {
            println!(
                "  head position: {:?}\n  tail position: {:?}",
                head_position, tail_position
            );
            let old_head_position = head_position;
            match direction {
                "R" => head_position.0 += 1.0,
                "L" => head_position.0 -= 1.0,
                "U" => head_position.1 += 1.0,
                "D" => head_position.1 -= 1.0,
                &_ => {}
            }
            println!("  head moved to {:?}", head_position);

            let q1 = (tail_position.0 - head_position.0 as f32).powi(2);
            let q2 = (tail_position.1 - head_position.1 as f32).powi(2);
            let head_tail_distance = (q1 + q2).sqrt();
            println!(
                "  d(H,T) = sqrt({:?} + {:?}) = {:?}",
                q1, q2, head_tail_distance
            );
            match head_tail_distance {
                x if x == 2.0 || x == (5 as f32).sqrt() => {
                    add_if_not_in(&mut tail_positions, old_head_position);
                    tail_position = old_head_position;
                    println!("  tail moved to {:?}", tail_position);
                },
                x if x == 1.0 || x == 0.0 || x == (2 as f32).sqrt() => println!("  tail not moved"),
                _ => panic!(
                    "unexpected dist between head and tail\n    head position: {:?}\n    tail position: {:?}\n    dist between them: {:?}",
                    head_position, tail_position, head_tail_distance
                ),
            }
            println!();
        }
    }

    println!("tail positions {}", tail_positions.len());
}
