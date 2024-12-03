use super::parse::{Input, Instruction};

pub fn solve(input: Input) -> u64 {
    input
        .instructions()
        .iter()
        .filter_map(|instruction| match instruction {
            Instruction::Multiplication(multiplication) => {
                Some(multiplication.0 * multiplication.1)
            }
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("src/day_3/test.txt").unwrap();
        let input = super::super::parse::parse(&input);
        assert_eq!(solve(input), 161);
    }
}
