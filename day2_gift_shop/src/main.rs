use std::fs::File;
use std::io::{self, BufRead};

const POW10: [i64; 20] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    i64::MAX,
];

fn main() {
    let file = File::open("input.txt").expect("No input file");
    let reader = io::BufReader::new(file);
    let ids: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let ids_set: Vec<&str> = ids.first().expect("IDs not found").split(',').collect();

    let invalid_id_sumup = ids_set
        .iter()
        .map(|f| filter_invalid_ids(f.trim()))
        .sum::<i64>();

    println!("Answer: {}", invalid_id_sumup)
}

fn filter_invalid_ids(ids_range: &str) -> i64 {
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

    (begin..=end).filter(|&n| is_invalid(n)).sum()
}

fn is_invalid(n: i64) -> bool {
    if n <= 10 {
        return false;
    }

    let length = (n as f64).log10() as u32 + 1;
    for part_length in 1..=length / 2 {
        if length.is_multiple_of(part_length) {
            let lowest = POW10[part_length as usize];
            let highest = POW10[(length - part_length) as usize];

            if n / lowest == n % highest {
                return true;
            }
        }
    }
    false
}
