# Advent of Code 2024 Solutions

This year, I've decided to tackle AoC in Rust 🦀

## How to run

Each day is a Cargo package, with part-1 and part-2 defined as binaries in each package. The main logic is written in `lib.rs` for each day (in the `process_part_one` and `process_part_two` functions).

To run, simply execute

```bash
cargo run --bin part-1
cargo run --bin part-2
```

I've added the toy examples given to you in AoC as tests, so running `cargo test` will execute the part-1 and part-2 functions as tests.