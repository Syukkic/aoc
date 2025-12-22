use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut current_point = 50;
    let mut zero_count = 0;
    let file = File::open("input.txt").expect("No input file");
    let reader = io::BufReader::new(file);
    let rotation_seq: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    rotation_seq.iter().for_each(|r| {
        let (direction, num_str) = r.split_at(1);
        let number = num_str.parse().ok().unwrap_or(0);

        match direction {
            "L" => {
                current_point -= number;
                while current_point < 0 {
                    current_point += 100
                }
            }
            "R" => {
                current_point += number;
                while current_point >= 100 {
                    current_point -= 100
                }
            }
            _ => panic!(),
        }
        if current_point == 0 {
            zero_count += 1
        }
    });

    println!("password: {}", zero_count);
}
