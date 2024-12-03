use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::{all_consuming, map, rest, value},
    error::Error,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Clone, Debug)]
pub struct Multiplication(pub u64, pub u64);

#[derive(Clone, Debug)]
pub enum Instruction {
    Multiplication(Multiplication),
    Do,
    Dont,
}

#[derive(Clone, Debug)]
pub struct Input {
    instructions: Vec<Instruction>,
}

impl Input {
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}

fn parse_multiplication(input: &str) -> IResult<&str, Instruction, Error<&str>> {
    delimited(
        tag("mul("),
        map(
            separated_pair(crate::parser::integer, char(','), crate::parser::integer),
            |(a, b)| Instruction::Multiplication(Multiplication(a, b)),
        ),
        tag(")"),
    )
    .parse(input)
}

fn parse_do(input: &str) -> IResult<&str, Instruction, Error<&str>> {
    value(Instruction::Do, tag("do()")).parse(input)
}

fn parse_dont(input: &str) -> IResult<&str, Instruction, Error<&str>> {
    value(Instruction::Dont, tag("don't()")).parse(input)
}

pub fn parse(input: &str) -> Input {
    let (_, result) = all_consuming::<_, _, Error<_>, _>(terminated(
        map(
            many1(map(
                many_till(anychar, alt((parse_multiplication, parse_do, parse_dont))),
                |(_, instruction)| instruction,
            )),
            |instructions| Input { instructions },
        ),
        rest,
    ))
    .parse(input)
    .unwrap();

    result
}
