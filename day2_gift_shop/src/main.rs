use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("No input file");
    let reader = io::BufReader::new(file);
    let ids: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let ids_set: Vec<&str> = ids.first().expect("IDs not found").split(',').collect();

    let mut invalid_id_sumup: i64 = 0;

    ids_set.iter().for_each(|f| {
        let result = filter_invalid_ids(f);
        invalid_id_sumup += result.iter().sum::<i64>();
    });

    println!("Answer: {}", invalid_id_sumup)
}

fn filter_invalid_ids(ids_range: &str) -> Vec<i64> {
    let mut parts = ids_range.split('-');
    let begin: i64 = parts
        .next()
        .expect("begining id not found")
        .parse()
        .ok()
        .unwrap_or(0);
    let end: i64 = parts
        .next()
        .expect("ending id not found")
        .parse()
        .ok()
        .unwrap_or(0);

    (begin..=end).filter(|&n| is_invalid(n)).collect()
}

fn is_invalid(n: i64) -> bool {
    if n <= 10 {
        return false;
    }
    let s = n.to_string();
    let bytes = s.as_bytes();
    let length = bytes.len();

    for part_length in 1..=length / 2 {
        if length.is_multiple_of(part_length) {
            let pattern = &bytes[0..part_length];
            let mut is_repeat = true;
            for chunk in bytes.chunks(part_length) {
                if chunk != pattern {
                    is_repeat = false;
                    break;
                }
            }
            if is_repeat {
                return true;
            };
        }
    }
    false
}
