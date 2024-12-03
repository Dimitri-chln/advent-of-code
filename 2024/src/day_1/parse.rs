use nom::{
    character::complete::{line_ending, space1},
    combinator::{all_consuming, map},
    error::Error,
    multi::many1,
    sequence::{separated_pair, terminated},
    Parser,
};

#[derive(Clone, Debug)]
pub struct Input {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl Input {
    pub fn left(&self) -> &[u64] {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut Vec<u64> {
        &mut self.left
    }

    pub fn right(&self) -> &[u64] {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut Vec<u64> {
        &mut self.right
    }
}

pub fn parse(input: &str) -> Input {
    let (_, result) = all_consuming::<_, _, Error<_>, _>(map(
        many1(terminated(
            separated_pair(crate::parser::integer, space1, crate::parser::integer),
            line_ending,
        )),
        |lists| {
            let (left, right): (Vec<_>, Vec<_>) = lists.into_iter().unzip();
            Input { left, right }
        },
    ))
    .parse(input)
    .unwrap();

    result
}
