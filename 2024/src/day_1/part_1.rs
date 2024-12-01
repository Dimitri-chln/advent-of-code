use super::parse::Input;

pub fn solve(mut input: Input) -> u64 {
    input.left_mut().sort();
    input.right_mut().sort();

    input
        .left()
        .iter()
        .zip(input.right())
        .map(|(&left, &right)| left.abs_diff(right))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("src/day_1/test.txt").unwrap();
        let input = super::super::parse::parse(&input);
        assert_eq!(solve(input), 11);
    }
}
