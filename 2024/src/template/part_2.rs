use super::parse::Input;

pub fn solve(mut input: Input) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("src/day_0/test.txt").unwrap();
        let input = super::super::parse::parse(&input);
        assert_eq!(solve(input), todo!());
    }
}
