use itertools::Itertools;

use crate::utils::skip_one::SkipOneIter;

use super::parse::Input;

pub fn solve(input: Input) -> u64 {
    input
        .reports()
        .iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                report
                    .iter()
                    .skip_one(i)
                    .tuple_windows()
                    .map(|(a, b)| a.cmp(b))
                    .all_equal()
                    && report
                        .iter()
                        .skip_one(i)
                        .tuple_windows()
                        .all(|(a, b)| a.abs_diff(**b) >= 1 && a.abs_diff(**b) <= 3)
            })
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("src/day_2/test.txt").unwrap();
        let input = super::super::parse::parse(&input);
        assert_eq!(solve(input), 4);
    }
}
