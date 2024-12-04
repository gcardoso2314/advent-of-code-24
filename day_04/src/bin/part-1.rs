use day_04::process_part_one;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part_one(&input))
}
