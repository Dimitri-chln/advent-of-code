use crate::utils::{direction::Direction, search_pattern::SearchPattern};

use super::parse::{Input, Letter};

pub fn solve(input: Input) -> u64 {
    input
        .letters()
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, _)| {
                    i < input.letters().len() - 2
                        && ((input.letters().search_pattern(
                            [Letter::M, Letter::A, Letter::S],
                            (i, *j),
                            Direction::DownRight,
                        ) || input.letters().search_pattern(
                            [Letter::S, Letter::A, Letter::M],
                            (i, *j),
                            Direction::DownRight,
                        )) && (input.letters().search_pattern(
                            [Letter::M, Letter::A, Letter::S],
                            (i + 2, *j),
                            Direction::UpRight,
                        ) || input.letters().search_pattern(
                            [Letter::S, Letter::A, Letter::M],
                            (i + 2, *j),
                            Direction::UpRight,
                        )))
                })
                .count() as u64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("src/day_4/test.txt").unwrap();
        let input = super::super::parse::parse(&input);
        assert_eq!(solve(input), 9);
    }
}
