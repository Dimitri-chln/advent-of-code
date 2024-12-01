use itertools::Itertools;

use super::parse::Input;

pub fn solve(input: Input) -> u64 {
    let counts = input.right().iter().counts();

    input
        .left()
        .iter()
        .map(|item| item * counts.get(item).copied().unwrap_or(0) as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("src/day_1/test.txt").unwrap();
        let input = super::super::parse::parse(&input);
        assert_eq!(solve(input), 31);
    }
}
