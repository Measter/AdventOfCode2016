#![allow(clippy::unnecessary_wraps)]

use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

#[derive(Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    fn parse(input: &str) -> Result<Vec<Vec<Instruction>>> {
        let mut digits = Vec::new();

        for line in input.lines().map(str::trim) {
            let digit = line
                .chars()
                .map(|c| match c {
                    'U' => Ok(Instruction::Up),
                    'D' => Ok(Instruction::Down),
                    'L' => Ok(Instruction::Left),
                    'R' => Ok(Instruction::Right),
                    _ => Err(eyre!("Unknown character: {}", c)),
                })
                .collect::<Result<_>>()?;
            digits.push(digit);
        }

        Ok(digits)
    }
}

fn part1(instrs: &[Vec<Instruction>]) -> Result<u32> {
    let mut number = 0;

    let mut x: u32 = 1;
    let mut y: u32 = 1;

    for digit_instrs in instrs {
        for instr in digit_instrs {
            match instr {
                Instruction::Up => y = y.saturating_sub(1),
                Instruction::Down => y = (y + 1).min(2),
                Instruction::Left => x = x.saturating_sub(1),
                Instruction::Right => x = (x + 1).min(2),
            }
        }

        let new_digit = y * 3 + x + 1;
        number = number * 10 + new_digit;
    }

    Ok(number)
}

fn part2(instrs: &[Vec<Instruction>]) -> Result<String> {
    #[rustfmt::skip]
    let grid = [
        ' ', ' ', '1', ' ', ' ',
        ' ', '2', '3', '4', ' ',
        '5', '6', '7', '8', '9',
        ' ', 'A', 'B', 'C', ' ',
        ' ', ' ', 'D', ' ', ' ',
    ];

    let mut number = String::new();

    let mut x: usize = 0;
    let mut y: usize = 2;

    for digit_instrs in instrs {
        for instr in digit_instrs {
            match instr {
                Instruction::Up => {
                    let maybe_new = y.saturating_sub(1);
                    if grid[maybe_new * 5 + x] != ' ' {
                        y = maybe_new;
                    }
                }
                Instruction::Down => {
                    let maybe_new = (y + 1).min(4);
                    if grid[maybe_new * 5 + x] != ' ' {
                        y = maybe_new;
                    }
                }
                Instruction::Left => {
                    let maybe_new = x.saturating_sub(1);
                    if grid[y * 5 + maybe_new] != ' ' {
                        x = maybe_new;
                    }
                }
                Instruction::Right => {
                    let maybe_new = (x + 1).min(4);
                    if grid[y * 5 + maybe_new] != ' ' {
                        x = maybe_new;
                    }
                }
            }
        }

        let new_digit = grid[y * 5 + x];
        number.push(new_digit);
    }

    Ok(number)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2016, 2).open()?;
    let (instructions, parse_bench) =
        aoc_lib::bench(&ALLOC, "Parse", &|| Instruction::parse(&input))?;

    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| part1(&instructions))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| part2(&instructions))?;

    aoc_lib::display_results(
        "Day 2: Bathroom Security",
        &[(&"", parse_bench), (&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

    Ok(())
}

#[cfg(test)]
mod tests_1602 {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "ULL
        RRDDD
        LURDL
        UUUUD";

        let instructions = Instruction::parse(input).unwrap();
        let expected = 1985;
        let actual = part1(&instructions).unwrap();

        assert_eq!(actual, expected);
    }
    #[test]
    fn part2_example() {
        let input = "ULL
        RRDDD
        LURDL
        UUUUD";

        let instructions = Instruction::parse(input).unwrap();
        let expected = "5DB3";
        let actual = part2(&instructions).unwrap();

        assert_eq!(actual, expected);
    }
}
