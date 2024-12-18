#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn char_to_move(char: char) -> Move {
    match char {
        '^' => Move::Up,
        '>' => Move::Right,
        'v' => Move::Down,
        '<' => Move::Left,
        _ => panic!("Invalid char encountered during move parsing"),
    }
}

fn parse_move_line(line: &str) -> Vec<Move> {
    line.chars().map(char_to_move).collect()
}

fn draw_grid(
    robot_pos: (usize, usize),
    walls: &Vec<(usize, usize)>,
    boxes: &Vec<(usize, usize)>,
    grid_w: usize,
    grid_h: usize,
) {
    for row in 0..grid_h {
        let mut row_str = String::from("");
        for col in 0..grid_w {
            if walls.contains(&(row, col)) || walls.contains(&(row, col - 1)) {
                row_str.push('#');
            } else if boxes.contains(&(row, col)) {
                row_str.push('[');
            } else if boxes.contains(&(row, col - 1)) {
                row_str.push(']');
            } else if robot_pos == (row, col) {
                row_str.push('@');
            } else {
                row_str.push('.');
            }
        }
        println!("{}", row_str);
    }
    println!("");
}

pub fn process_part_one(input: &str) -> usize {
    let mut boxes: Vec<(usize, usize)> = Vec::new();
    let mut walls: Vec<(usize, usize)> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();
    let mut robot_pos: (usize, usize) = (0, 0);

    input
        .lines()
        .enumerate()
        .for_each(|(row, line)| match line {
            line if line.starts_with('#') => {
                line.chars().enumerate().for_each(|(col, c)| match c {
                    'O' => boxes.push((row, col)),
                    '@' => robot_pos = (row, col),
                    '#' => walls.push((row, col)),
                    _ => (),
                });
            }
            line if line.starts_with(&['^', '>', '<', 'v']) => moves.extend(parse_move_line(line)),
            _ => (),
        });

    for mv in moves.iter() {
        boxes.sort();
        match mv {
            Move::Left => {
                let (row, mut col) = robot_pos;

                let filtered_walls: Vec<&(usize, usize)> =
                    walls.iter().filter(|w| w.0 == row && w.1 < col).collect();
                let nearest_wall = filtered_walls.last().unwrap().1;
                let mut filtered_boxes: Vec<&mut (usize, usize)> = boxes
                    .iter_mut()
                    .filter(|b| b.0 == row && b.1 < col && b.1 > nearest_wall)
                    .collect();

                if filtered_boxes.len() < col - nearest_wall - 1 {
                    for b in filtered_boxes.iter_mut().rev() {
                        if b.1 != col - 1 {
                            // no more boxes
                            break;
                        } else {
                            b.1 -= 1;
                        }
                        col -= 1;
                    }
                    robot_pos.1 -= 1;
                }
            }
            Move::Right => {
                let (row, mut col) = robot_pos;

                let filtered_walls: Vec<&(usize, usize)> =
                    walls.iter().filter(|w| w.0 == row && w.1 > col).collect();
                let nearest_wall = filtered_walls.first().unwrap().1;
                let mut filtered_boxes: Vec<&mut (usize, usize)> = boxes
                    .iter_mut()
                    .filter(|b| b.0 == row && b.1 > col && b.1 < nearest_wall)
                    .collect();
                if filtered_boxes.len() < nearest_wall - col - 1 {
                    for b in filtered_boxes.iter_mut() {
                        if b.1 != col + 1 {
                            // no more boxes
                            break;
                        } else {
                            b.1 += 1;
                        }
                        col += 1;
                    }
                    robot_pos.1 += 1;
                }
            }
            Move::Up => {
                let (mut row, col) = robot_pos;

                let filtered_walls: Vec<&(usize, usize)> =
                    walls.iter().filter(|w| w.0 < row && w.1 == col).collect();
                let nearest_wall = filtered_walls.last().unwrap().0;
                let mut filtered_boxes: Vec<&mut (usize, usize)> = boxes
                    .iter_mut()
                    .filter(|b| b.0 < row && b.0 > nearest_wall && b.1 == col)
                    .collect();
                if filtered_boxes.len() < row - nearest_wall - 1 {
                    for b in filtered_boxes.iter_mut().rev() {
                        if b.0 != row - 1 {
                            // no more boxes
                            break;
                        } else {
                            b.0 -= 1;
                        }
                        row -= 1;
                    }
                    robot_pos.0 -= 1;
                }
            }
            Move::Down => {
                let (mut row, col) = robot_pos;

                let filtered_walls: Vec<&(usize, usize)> =
                    walls.iter().filter(|w| w.0 > row && w.1 == col).collect();
                let nearest_wall = filtered_walls.first().unwrap().0;
                let mut filtered_boxes: Vec<&mut (usize, usize)> = boxes
                    .iter_mut()
                    .filter(|b| b.0 > row && b.0 < nearest_wall && b.1 == col)
                    .collect();
                if filtered_boxes.len() < nearest_wall - row - 1 {
                    for b in filtered_boxes.iter_mut() {
                        if b.0 != row + 1 {
                            // no more boxes
                            break;
                        } else {
                            b.0 += 1;
                        }
                        row += 1;
                    }
                    robot_pos.0 += 1;
                }
            }
        }
    }

    boxes.iter().map(|b| 100 * b.0 + b.1).sum()
}

fn get_boxes_vertical<'a>(
    boxes: &'a mut Vec<(usize, usize)>,
    walls: &'a Vec<(usize, usize)>,
    robot_pos: (usize, usize),
    grid_h: usize,
    dir: Move,
) -> Option<Vec<&'a mut (usize, usize)>> {
    let (mut row, col) = robot_pos;
    let (mut min, mut max) = (col - 1, col);
    let mut box_indices: Vec<usize> = Vec::new();
    let mut can_move = false;
    match dir {
        Move::Up => row -= 1,
        Move::Down => row += 1,
        _ => panic!("Expected only up or down move"),
    }
    while row > 0 && row < grid_h {
        if walls.iter().any(|&(r, c)| r == row && c >= min && c <= max) {
            return None;
        }
        let boxes_above: Vec<usize> = boxes
            .iter()
            .enumerate()
            .filter(|&(_, &(r, c))| r == row && c >= min && c <= max)
            .map(|(i, _)| i)
            .collect();
        if boxes_above.len() == 0 {
            can_move = true;
            break;
        }
        min = boxes_above.iter().map(|&i| boxes[i].1).min().unwrap() - 1;
        max = boxes_above.iter().map(|&i| boxes[i].1).max().unwrap() + 1;
        box_indices.extend(boxes_above);

        match dir {
            Move::Up => row -= 1,
            Move::Down => row += 1,
            _ => panic!("Expected only up or down move"),
        }
    }
    if can_move {
        Some(
            boxes
                .iter_mut()
                .enumerate()
                .filter_map(|(i, b)| {
                    if box_indices.contains(&i) {
                        Some(b)
                    } else {
                        None
                    }
                })
                .collect(),
        )
    } else {
        None
    }
}

fn get_boxes_horizontal<'a>(
    boxes: &'a mut Vec<(usize, usize)>,
    walls: &'a Vec<(usize, usize)>,
    robot_pos: (usize, usize),
    grid_w: usize,
    dir: Move,
) -> Option<Vec<&'a mut (usize, usize)>> {
    let (row, mut col) = robot_pos;
    let mut box_indices: Vec<usize> = Vec::new();
    let mut can_move = false;
    match dir {
        Move::Left => col -= 1,
        Move::Right => col += 1,
        _ => panic!("Expected only left or right move"),
    }
    while col > 1 && col < grid_w {
        if walls
            .iter()
            .any(|&(r, c)| r == row && (c == col || c == col - 1))
        {
            return None;
        }
        let boxes_above: Vec<usize> = boxes
            .iter()
            .enumerate()
            .filter(|&(_, &(r, c))| r == row && (c == col || c == col - 1))
            .map(|(i, _)| i)
            .collect();
        if boxes_above.len() == 0 {
            can_move = true;
            break;
        }
        box_indices.extend(boxes_above);

        match dir {
            Move::Left => col -= 1,
            Move::Right => col += 1,
            _ => panic!("Expected only left or right move"),
        }
    }

    if can_move {
        Some(
            boxes
                .iter_mut()
                .enumerate()
                .filter_map(|(i, b)| {
                    if box_indices.contains(&i) {
                        Some(b)
                    } else {
                        None
                    }
                })
                .collect(),
        )
    } else {
        None
    }
}

pub fn process_part_two(input: &str) -> usize {
    let mut boxes: Vec<(usize, usize)> = Vec::new();
    let mut walls: Vec<(usize, usize)> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();
    let mut robot_pos: (usize, usize) = (0, 0);
    let mut grid_h = 0;
    let grid_w = input.lines().next().unwrap().len() * 2;

    input
        .lines()
        .enumerate()
        .for_each(|(row, line)| match line {
            line if line.starts_with('#') => {
                grid_h += 1;
                let mut extra_counts = 0;
                line.chars().enumerate().for_each(|(col, c)| {
                    match c {
                        'O' => boxes.push((row, col + extra_counts)),
                        '@' => robot_pos = (row, col + extra_counts),
                        '#' => walls.push((row, col + extra_counts)),
                        _ => (),
                    };
                    extra_counts += 1
                });
            }
            line if line.starts_with(&['^', '>', '<', 'v']) => moves.extend(parse_move_line(line)),
            _ => (),
        });

    for (_, mv) in moves.iter().enumerate() {
        boxes.sort();
        match mv {
            Move::Left => {
                let filtered_boxes =
                    get_boxes_horizontal(&mut boxes, &walls, robot_pos, grid_w, Move::Left);
                if let Some(mut bx) = filtered_boxes {
                    for b in bx.iter_mut() {
                        b.1 -= 1;
                    }
                    robot_pos.1 -= 1;
                }
            }
            Move::Right => {
                let filtered_boxes =
                    get_boxes_horizontal(&mut boxes, &walls, robot_pos, grid_w, Move::Right);
                if let Some(mut bx) = filtered_boxes {
                    for b in bx.iter_mut() {
                        b.1 += 1;
                    }
                    robot_pos.1 += 1;
                }
            }
            Move::Up => {
                let filtered_boxes =
                    get_boxes_vertical(&mut boxes, &walls, robot_pos, grid_h, Move::Up);
                if let Some(mut bx) = filtered_boxes {
                    for b in bx.iter_mut() {
                        b.0 -= 1;
                    }
                    robot_pos.0 -= 1;
                }
            }
            Move::Down => {
                let filtered_boxes =
                    get_boxes_vertical(&mut boxes, &walls, robot_pos, grid_h, Move::Down);
                if let Some(mut bx) = filtered_boxes {
                    for b in bx.iter_mut() {
                        b.0 += 1;
                    }
                    robot_pos.0 += 1;
                }
            }
        }
    }

    boxes.iter().map(|b| 100 * b.0 + b.1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 10092)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 9021)
    }
}
