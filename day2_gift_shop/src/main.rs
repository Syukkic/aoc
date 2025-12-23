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

    println!("Password: {}", invalid_id_sumup)
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

    (begin..=end)
        .filter(|&n| {
            let s = n.to_string();
            let length = s.len();
            length % 2 == 0 && s.split_at(length / 2).0 == s.split_at(length / 2).1
        })
        .collect()
}
