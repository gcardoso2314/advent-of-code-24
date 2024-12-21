use day_16::process_part_one;
use std::fs;

fn main() {
    let start = std::time::Instant::now();
    let input = fs::read_to_string("input.txt").expect("error loading the input file");
    println!("{}", process_part_one(&input));
    println!("{:.2?}", start.elapsed());
}
