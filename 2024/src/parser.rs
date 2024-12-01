use nom::{character::complete::digit1, combinator::map_res, error::Error, IResult, Parser};

pub fn integer(input: &str) -> IResult<&str, u64, Error<&str>> {
    map_res(digit1, str::parse::<u64>).parse(input)
}
