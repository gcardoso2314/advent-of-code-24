use day_18::process_part_two;
use std::fs;

fn main() {
    let start = std::time::Instant::now();
    let input = fs::read_to_string("input.txt").expect("error loading the input file");
    println!("{}", process_part_two(&input, 71));
    println!("{:.2?}", start.elapsed());
}
