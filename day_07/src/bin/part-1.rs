use day_07::process_part_one;
use std::fs;

fn main() {
    let start = std::time::Instant::now();
    let input = fs::read("input.txt").unwrap();
    println!("{}", process_part_one(&input));
    println!("{:.2?}", start.elapsed());
}
