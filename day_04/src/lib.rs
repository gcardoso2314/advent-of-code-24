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

fn get_char(grid: &Vec<Vec<char>>, pos: (usize, usize), dir: (i32, i32)) -> Option<&char> {
    let row1 = pos.0 as i32 + dir.0;
    let col1 = pos.1 as i32 + dir.1;
    match grid.get(row1 as usize) {
        Some(v) => v.get(col1 as usize),
        None => None,
    }
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
                for d in vec![d1, d2] {
                    if let Some(c) = get_char(&grid, (i, j), *d) {
                        chars.push(*c)
                    }
                }

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
