use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("No input file");
    let reader = io::BufReader::new(file);

    let banks: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    // let banks = [
    //     "987654321111111",
    //     "811111111111119",
    //     "234234234234278",
    //     "818181911112111",
    // ];

    let largest_battery = banks.iter().map(|f| find_the_largest(f)).sum::<i32>();
    println!("Answer: {}", largest_battery)
}

fn find_the_largest(input: &str) -> i32 {
    let digits: Vec<i32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0) as i32)
        .collect();

    let mut left_p = 0;
    let mut max_result = 0;

    for n in 1..digits.len() {
        let temp_sumup = digits[left_p] * 10 + digits[n];
        if temp_sumup > max_result {
            max_result = temp_sumup
        }

        if digits[n] > digits[left_p] {
            left_p = n
        }
    }

    max_result
}
