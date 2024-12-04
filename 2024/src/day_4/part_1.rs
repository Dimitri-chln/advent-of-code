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
                .map(|(j, _)| {
                    Direction::all()
                        .iter()
                        .filter(|&&direction| {
                            input.letters().search_pattern(
                                [Letter::X, Letter::M, Letter::A, Letter::S],
                                (i, j),
                                direction,
                            )
                        })
                        .count() as u64
                })
                .sum::<u64>()
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
        assert_eq!(solve(input), 18);
    }
}
