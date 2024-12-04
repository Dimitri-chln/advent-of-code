use std::{fs, io::Result};

mod parse;
mod part_1;
mod part_2;
mod structs;

pub fn solve() -> Result<(u64, u64)> {
    let input = fs::read_to_string("src/day_4/input.txt")?;
    let input = parse::parse(&input);

    let solution_1 = part_1::solve(input.clone());
    let solution_2 = part_2::solve(input.clone());

    Ok((solution_1, solution_2))
}
