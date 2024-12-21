use day_18::process_part_one;
use std::fs;

fn main() {
    let start = std::time::Instant::now();
    let input = fs::read_to_string("input.txt").expect("error loading the input file");
    println!("{}", process_part_one(&input, 71, 1024));
    println!("{:.2?}", start.elapsed());
}
