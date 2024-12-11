use std::collections::VecDeque;

#[derive(Debug)]
enum Block {
    File { file_id: usize, blocks: u32 },
    FreeSpace(u32),
}

pub fn process_part_one(input: &str) -> usize {
    let mut stack: VecDeque<Block> = VecDeque::new();
    let mut file_id = 0;
    input.chars().enumerate().for_each(|(i, c)| {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                stack.push_back(Block::File {
                    file_id,
                    blocks: digit,
                });
                file_id += 1;
            } else {
                stack.push_back(Block::FreeSpace(digit))
            }
        }
    });

    let mut filesystem: Vec<usize> = Vec::new();
    while !stack.is_empty() {
        let block = stack.pop_front().unwrap();
        match block {
            Block::File { file_id, blocks } => {
                for _ in 0..blocks {
                    filesystem.push(file_id)
                }
            }
            Block::FreeSpace(n) => {
                let mut blocks_to_fill = n;
                while blocks_to_fill > 0 {
                    if let Some(end_block) = stack.pop_back() {
                        match end_block {
                            Block::File { file_id, blocks } => {
                                if blocks <= blocks_to_fill {
                                    blocks_to_fill -= blocks;
                                    for _ in 0..blocks {
                                        filesystem.push(file_id)
                                    }
                                } else {
                                    stack.push_back(Block::File {
                                        file_id,
                                        blocks: blocks - blocks_to_fill,
                                    });
                                    for _ in 0..blocks_to_fill {
                                        filesystem.push(file_id)
                                    }
                                    blocks_to_fill = 0;
                                }
                            }
                            Block::FreeSpace(_) => continue,
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    filesystem.iter().enumerate().map(|(i, n)| i * n).sum()
}

pub fn process_part_two(input: &str) -> usize {
    let mut stack: Vec<Block> = Vec::new();
    let mut file_id = 0;
    input.chars().enumerate().for_each(|(i, c)| {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                stack.push(Block::File {
                    file_id,
                    blocks: digit,
                });
                file_id += 1;
            } else {
                stack.push(Block::FreeSpace(digit))
            }
        }
    });

    let mut filesystem: Vec<usize> = Vec::new();
    while let Some(block) = stack.pop() {
        match block {
            Block::File { file_id, blocks } => {
                // check if there's a space to put it in the stack
                let mut found: bool = false;
                let mut insert_idx = 0;
                let mut free_space_blocks = 0;
                for (i, b) in stack.iter().enumerate() {
                    match b {
                        Block::File {
                            file_id: _,
                            blocks: _,
                        } => continue,
                        Block::FreeSpace(n) => {
                            if blocks <= *n {
                                found = true;
                                insert_idx = i;
                                free_space_blocks = *n;
                                break;
                            }
                        }
                    }
                }
                if found {
                    stack.remove(insert_idx);
                    stack.insert(insert_idx, Block::File { file_id, blocks });
                    stack.insert(insert_idx + 1, Block::FreeSpace(free_space_blocks - blocks));

                    // Add free spaces that were left behind
                    stack.push(Block::FreeSpace(blocks));
                } else {
                    for _ in 0..blocks {
                        filesystem.push(file_id)
                    }
                }
            }
            Block::FreeSpace(n) => {
                // Add it back in, don't care
                for _ in 0..n {
                    filesystem.push(0)
                }
            }
        }
    }

    filesystem
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| i * n)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 1928)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 2858)
    }
}
