use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let current_point = 50;
    let file = File::open("input.txt").expect("No input file");
    let reader = io::BufReader::new(file);
    let rotation_seq: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();
    let zero_count = calc_password(current_point, rotation_seq);

    println!("password: {}", zero_count);
}

fn calc_password(mut current_point: i32, rotation_seq: Vec<String>) -> i32 {
    let mut zero_count = 0;

    rotation_seq.iter().for_each(|r| {
        let (direction, num_str) = r.split_at(1);
        let steps = num_str.parse().ok().unwrap_or(0);

        match direction {
            "L" => {
                let first_zero = if current_point % 100 == 0 {
                    100
                } else {
                    current_point % 100
                };
                if first_zero <= steps {
                    zero_count += 1 + (steps - first_zero) / 100;
                }

                current_point = (current_point - steps).rem_euclid(100)
            }
            "R" => {
                let steps_to_first_zero = (100 - current_point) % 100;
                let first_zero = if steps_to_first_zero == 0 {
                    100
                } else {
                    steps_to_first_zero
                };

                if first_zero <= steps {
                    zero_count += 1 + (steps - first_zero) / 100;
                }

                current_point = (current_point + steps).rem_euclid(100)
            }
            _ => unreachable!(),
        }
    });

    zero_count
}
