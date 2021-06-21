use std::{collections::VecDeque, num::ParseIntError};

use aoc_lib::{parsers::unsigned_number, TracingAlloc};
use color_eyre::Result;
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

#[derive(Debug, Clone, Copy)]
enum TransferDest {
    Bot(usize),
    Output(usize),
}

enum Instruction {
    Init {
        bot: usize,
        value: usize,
    },
    Transfer {
        low: TransferDest,
        high: TransferDest,
    },
}

impl Instruction {
    fn parse(input: &str) -> Result<VecDeque<Instruction>> {
        let instrs = VecDeque::new();

        let bot_parser = map(
            tuple((tag("bot "), unsigned_number::<usize>)),
            |(_, num)| num,
        );
        let output_parser = map(
            tuple((tag("output "), unsigned_number::<usize>)),
            |(_, num)| num,
        );

        let init = 

        for line in input.lines() {}

        Ok(instrs)
    }
}

struct Bot {
    id: usize,
    left: Option<usize>,
    right: Option<usize>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2016, 9).open()?;

    aoc_lib::display_results("Day 10: Balance Bots", &[]);

    Ok(())
}

#[cfg(test)]
mod tests_1610 {
    use super::*;
}
