use nom::{
    character::complete::line_ending,
    combinator::{all_consuming, map},
    multi::many1,
    sequence::terminated,
    Parser,
};

#[derive(Clone, Debug)]
pub struct Input {}

pub fn parse(input: &str) -> Input {
    let (_, result) = all_consuming::<_, _, nom::error::Error<_>, _>(map(
        many1(terminated(crate::parser::integer, line_ending)),
        |_| Input {},
    ))
    .parse(input)
    .unwrap();

    result
}
