use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, u32},
    combinator::map,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

enum Command {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_mul(input: &str) -> IResult<&str, Command> {
    map(
        delimited(tag("mul("), separated_pair(u32, char(','), u32), char(')')),
        |(a, b)| Command::Mul(a, b),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    let parse_do = tag("do()").map(|_| Command::Do);
    let parse_dont = tag("don't()").map(|_| Command::Dont);
    let (input, out) = many1(many_till(anychar, alt((parse_do, parse_dont, parse_mul))))(input)?;
    let (_, cmds): (Vec<Vec<char>>, Vec<Command>) = out.into_iter().unzip();
    Ok((input, cmds))
}

pub fn process_part_one(input: &str) -> u32 {
    let (_, cmds) = parse_input(input).unwrap();
    cmds.iter()
        .filter_map(|cmd| match cmd {
            Command::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum()
}

pub fn process_part_two(input: &str) -> u32 {
    let (_, cmds) = parse_input(input).unwrap();
    let mut active: bool = true;
    let mut result: u32 = 0;
    cmds.iter().for_each(|cmd| match cmd {
        Command::Do => active = true,
        Command::Dont => active = false,
        Command::Mul(a, b) => result += a * b * active as u32,
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 161)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT_2), 48)
    }
}
