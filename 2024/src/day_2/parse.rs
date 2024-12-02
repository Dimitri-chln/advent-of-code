use nom::{
    character::complete::{line_ending, space1},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::terminated,
    Parser,
};

#[derive(Clone, Debug)]
pub struct Level(u64);

impl std::ops::Deref for Level {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct Report(Vec<Level>);

impl std::ops::Deref for Report {
    type Target = Vec<Level>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    reports: Vec<Report>,
}

impl Input {
    pub fn reports(&self) -> &[Report] {
        &self.reports
    }
}

pub fn parse(input: &str) -> Input {
    let (_, result) = all_consuming::<_, _, nom::error::Error<_>, _>(map(
        many1(terminated(
            map(
                separated_list1(space1, map(crate::parser::integer, Level)),
                Report,
            ),
            line_ending,
        )),
        |reports| Input { reports },
    ))
    .parse(input)
    .unwrap();

    result
}
