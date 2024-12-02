use day_02::process_part_two;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part_two(&input))
}
