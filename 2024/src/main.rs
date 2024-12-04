use std::io::Result;

use advent_of_code_2024::{day_1, day_2, day_3, day_4};
use clap::Parser;

#[derive(Parser)]
struct Args {
    day: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (solution_1, solution_2) = match args.day {
        1 => day_1::solve()?,
        2 => day_2::solve()?,
        3 => day_3::solve()?,
        4 => day_4::solve()?,
        _ => unimplemented!(),
    };

    println!(
        "Solutions of day {}:\n\tPart one: {}\n\tPart two: {}",
        args.day, solution_1, solution_2
    );

    Ok(())
}
