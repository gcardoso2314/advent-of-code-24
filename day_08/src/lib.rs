use itertools::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Antenna {
    row: i32,
    col: i32,
}

fn find_antinode_pair(a1: &Antenna, a2: &Antenna) -> ((i32, i32), (i32, i32)) {
    let diff = (a2.row - a1.row, a2.col - a1.col);

    (
        (a1.row - diff.0, a1.col - diff.1),
        (a2.row + diff.0, a2.col + diff.1),
    )
}

fn find_all_antinodes(a1: &Antenna, a2: &Antenna, grid_h: usize, grid_w: usize) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    let diff = (a2.row - a1.row, a2.col - a1.col);

    antinodes.push((a1.row, a1.col));
    antinodes.push((a2.row, a2.col));

    // Find in one direction
    let mut an = (a1.row, a1.col);
    loop {
        let (nrow, ncol) = (an.0 - diff.0, an.1 - diff.1);
        if nrow >= 0 && nrow < grid_h as i32 && ncol >= 0 && ncol < grid_w as i32 {
            antinodes.push((nrow, ncol));
            an = (nrow, ncol);
        } else {
            break;
        }
    }

    // Find in other direction
    an = (a2.row, a2.col);
    loop {
        let (nrow, ncol) = (an.0 + diff.0, an.1 + diff.1);
        if nrow >= 0 && nrow < grid_h as i32 && ncol >= 0 && ncol < grid_w as i32 {
            antinodes.push((nrow, ncol));
            an = (nrow, ncol);
        } else {
            break;
        }
    }
    antinodes
}

pub fn process_part_one(input: &str) -> usize {
    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| match c {
            '.' => (),
            _ => {
                antennas
                    .entry(c)
                    .and_modify(|v| {
                        v.push(Antenna {
                            row: row as i32,
                            col: col as i32,
                        })
                    })
                    .or_insert(vec![Antenna {
                        row: row as i32,
                        col: col as i32,
                    }]);
            }
        });
    });

    let grid_w = lines.len();
    let grid_h = lines[0].len();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    antennas.values().for_each(|ants| {
        ants.iter().combinations(2).for_each(|v| {
            let (p1, p2) = find_antinode_pair(v[0], v[1]);
            antinodes.insert(p1);
            antinodes.insert(p2);
        })
    });

    antinodes
        .iter()
        .filter(|(row, col)| {
            row >= &0 && row < &(grid_h as i32) && col >= &0 && col < &(grid_w as i32)
        })
        .count()
}

pub fn process_part_two(input: &str) -> usize {
    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| match c {
            '.' => (),
            _ => {
                antennas
                    .entry(c)
                    .and_modify(|v| {
                        v.push(Antenna {
                            row: row as i32,
                            col: col as i32,
                        })
                    })
                    .or_insert(vec![Antenna {
                        row: row as i32,
                        col: col as i32,
                    }]);
            }
        });
    });

    let grid_w = lines.len();
    let grid_h = lines[0].len();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    antennas.values().for_each(|ants| {
        ants.iter()
            .combinations(2)
            .for_each(|v| antinodes.extend(find_all_antinodes(v[0], v[1], grid_h, grid_w)))
    });

    // draw_grid(input, &antinodes);
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 14)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 34)
    }
}
