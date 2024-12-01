use nom::{
    character::complete::digit1,
    combinator::{all_consuming, map},
    Parser,
};

#[derive(Clone, Debug)]
pub struct Input {}

pub fn parse(input: &str) -> Input {
    let (_, result) = all_consuming::<_, _, nom::error::Error<_>, _>(map(digit1, |_| Input {}))
        .parse(input)
        .unwrap();

    result
}
