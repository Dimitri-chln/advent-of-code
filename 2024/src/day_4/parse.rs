use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{all_consuming, map, value},
    multi::many1,
    sequence::terminated,
    Parser,
};

#[derive(PartialEq, Clone, Debug)]
pub enum Letter {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Debug)]
pub struct Input {
    letters: Vec<Vec<Letter>>,
}

impl Input {
    pub fn letters(&self) -> &[Vec<Letter>] {
        &self.letters
    }
}

pub fn parse(input: &str) -> Input {
    let (_, result) = all_consuming::<_, _, nom::error::Error<_>, _>(map(
        many1(terminated(
            many1(alt((
                value(Letter::X, char('X')),
                value(Letter::M, char('M')),
                value(Letter::A, char('A')),
                value(Letter::S, char('S')),
            ))),
            line_ending,
        )),
        |letters| Input { letters },
    ))
    .parse(input)
    .unwrap();

    result
}
