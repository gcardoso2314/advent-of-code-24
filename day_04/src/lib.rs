fn check_word(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    dir: (i32, i32),
    letter_idx: usize,
) -> bool {
    if letter_idx == 3 {
        return true;
    }

    let row = pos.0 as i32 + dir.0;
    let col = pos.1 as i32 + dir.1;

    let result = match grid.get(row as usize) {
        Some(v) => match v.get(col as usize) {
            Some(c) => *c == "XMAS".chars().nth(letter_idx + 1).unwrap(),
            None => false,
        },
        None => false,
    };

    // If matched whole word then continue recursing
    if result {
        check_word(grid, (row as usize, col as usize), dir, letter_idx + 1)
    } else {
        false
    }
}

pub fn process_part_one(input: &str) -> usize {
    let directions = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut num_found = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val != 'X' {
                continue;
            }

            directions.iter().for_each(|dir| {
                if check_word(&grid, (i, j), *dir, 0) {
                    num_found += 1
                }
            })
        }
    }

    num_found
}

pub fn process_part_two(input: &str) -> usize {
    let diagonals = [((-1, -1), (1, 1)), ((-1, 1), (1, -1))];
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut num_found = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val != 'A' {
                continue;
            }

            let mut found: bool = true;
            diagonals.iter().for_each(|(d1, d2)| {
                let mut chars: Vec<char> = Vec::new();
                // check d1
                let row1 = i as i32 + d1.0;
                let col1 = j as i32 + d1.1;
                match grid.get(row1 as usize) {
                    Some(v) => match v.get(col1 as usize) {
                        Some(c) => chars.push(*c),
                        None => (),
                    },
                    None => (),
                };

                // check d2
                let row2 = i as i32 + d2.0;
                let col2 = j as i32 + d2.1;
                match grid.get(row2 as usize) {
                    Some(v) => match v.get(col2 as usize) {
                        Some(c) => chars.push(*c),
                        None => (),
                    },
                    None => (),
                };

                found &= chars.contains(&'M') && chars.contains(&'S')
            });

            num_found += found as usize;
        }
    }

    num_found
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 18)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 9)
    }
}
