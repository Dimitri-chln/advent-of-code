use super::parse::{Input, Instruction};

pub fn solve(input: Input) -> u64 {
    let mut enabled = true;

    input
        .instructions()
        .iter()
        .filter_map(|instruction| {
            match instruction {
                Instruction::Multiplication(multiplication) => {
                    return enabled.then_some(multiplication.0 * multiplication.1)
                }
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
            }

            None
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
        assert_eq!(solve(input), 48);
    }
}
