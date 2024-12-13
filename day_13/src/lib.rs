use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, i64, newline},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: (i64, i64),
}

fn parse_button(input: &str) -> IResult<&str, Button> {
    let (input, (x, y)) = tuple((
        preceded(tuple((tag("Button "), alpha1, tag(": X+"))), i64),
        preceded(tag(", Y+"), i64),
    ))(input)?;

    Ok((input, Button { x, y }))
}

fn parse_machine(input: &str) -> IResult<&str, ClawMachine> {
    let (input, (button_a, button_b)) = separated_pair(parse_button, newline, parse_button)(input)?;
    let (input, _) = newline(input)?;
    let (input, (x, y)) =
        tuple((preceded(tag("Prize: X="), i64), preceded(tag(", Y="), i64)))(input)?;

    Ok((
        input,
        ClawMachine {
            button_a,
            button_b,
            prize: (x, y),
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    separated_list1(many1(newline), parse_machine)(input)
}

pub fn process_part_one(input: &str) -> i64 {
    let (_, machines) = parse_input(input).unwrap();
    machines
        .iter()
        .map(|mach| {
            let (prize_x, prize_y) = mach.prize;
            let (button_a, button_b) = (&mach.button_a, &mach.button_b);
            let a_presses = (prize_x * button_b.y - prize_y * button_b.x)
                / (button_a.x * button_b.y - button_a.y * button_b.x);
            let b_presses = (prize_y - a_presses * button_a.y) / button_b.y;

            if a_presses >= 0
                && a_presses <= 100
                && b_presses >= 0
                && b_presses <= 100
                && (button_a.x * a_presses + button_b.x * b_presses) == prize_x
                && (button_a.y * a_presses + button_b.y * b_presses) == prize_y
            {
                a_presses * 3 + b_presses
            } else {
                0
            }
        })
        .sum()
}

pub fn process_part_two(input: &str) -> i64 {
    let (_, machines) = parse_input(input).unwrap();
    machines
        .iter()
        .map(|mach| {
            let (prize_x, prize_y) = (mach.prize.0 + 10000000000000, mach.prize.1 + 10000000000000);
            let (button_a, button_b) = (&mach.button_a, &mach.button_b);
            let a_presses = (prize_x * button_b.y - prize_y * button_b.x)
                / (button_a.x * button_b.y - button_a.y * button_b.x);
            let b_presses = (prize_y - a_presses * button_a.y) / button_b.y;

            if a_presses >= 0
                && b_presses >= 0
                && (button_a.x * a_presses + button_b.x * b_presses) == prize_x
                && (button_a.y * a_presses + button_b.y * b_presses) == prize_y
            {
                a_presses * 3 + b_presses
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), 480)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 875318608908)
    }
}
