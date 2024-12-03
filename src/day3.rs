use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Clone)]
pub enum Command {
    Mult(isize, isize),
    Do,
    Dont,
}
type Program = Vec<Command>;

fn mul(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Command::Mult(pair.0 as isize, pair.1 as isize)))
}
fn get_instruction(input: &str) -> IResult<&str, Command> {
    alt((
        value(Command::Do, tag("do()")),
        value(Command::Dont, tag("don't()")),
        mul,
    ))(input)
}
fn parse(input: &str) -> IResult<&str, Program> {
    many1(many_till(anychar, get_instruction).map(|(_b, ins)| ins))(input)
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Program {
    let (_in, program) = parse(input).expect("parse failed");
    return program;
}

#[aoc(day3, part1)]
pub fn solve_part1(program: &Program) -> isize {
    program
        .iter()
        .filter_map(|command| match command {
            Command::Do | Command::Dont => None,
            Command::Mult(v1, v2) => Some(v1 * v2),
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(program: &Program) -> isize {
    let mut do_mult = true;
    let mut total = 0;
    for command in program {
        match command {
            Command::Do => do_mult = true,
            Command::Dont => do_mult = false,
            Command::Mult(v1, v2) => {
                if do_mult {
                    total += v1 * v2;
                }
            }
        }
    }
    return total;
}
