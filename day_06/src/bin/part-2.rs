use day_06::process_part_two;
use std::fs;

fn main() {
    let start = std::time::Instant::now();
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part_two(&input));
    println!("{:.2?}", start.elapsed());
}
