#![allow(clippy::unnecessary_wraps)]

use std::collections::HashSet;

use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Left(i16),
    Right(i16),
}

impl Instruction {
    fn parse(input: &str) -> Result<Vec<Instruction>> {
        input
            .split(", ")
            .map(str::trim)
            .map(|i| {
                let (dir, len) = i.split_at(1);
                match dir {
                    "L" => Ok(Instruction::Left(len.parse()?)),
                    "R" => Ok(Instruction::Right(len.parse()?)),
                    u => Err(eyre!("Unknown direction: {}", u)),
                }
            })
            .collect()
    }
}

#[derive(Debug, Copy, Clone)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn turn_left(self) -> Self {
        use Heading::*;
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    fn turn_right(self) -> Self {
        use Heading::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

fn part1(instrs: &[Instruction]) -> Result<i16> {
    let (_, (x, y)) =
        instrs
            .iter()
            .fold((Heading::North, (0, 0)), |(heading, (x, y)), next| {
                match (heading, *next) {
                    (Heading::North, Instruction::Left(distance)) => {
                        (heading.turn_left(), (x - distance, y))
                    }
                    (Heading::North, Instruction::Right(distance)) => {
                        (heading.turn_right(), (x + distance, y))
                    }
                    (Heading::East, Instruction::Left(distance)) => {
                        (heading.turn_left(), (x, y + distance))
                    }
                    (Heading::East, Instruction::Right(distance)) => {
                        (heading.turn_right(), (x, y - distance))
                    }
                    (Heading::South, Instruction::Left(distance)) => {
                        (heading.turn_left(), (x + distance, y))
                    }
                    (Heading::South, Instruction::Right(distance)) => {
                        (heading.turn_right(), (x - distance, y))
                    }
                    (Heading::West, Instruction::Left(distance)) => {
                        (heading.turn_left(), (x, y - distance))
                    }
                    (Heading::West, Instruction::Right(distance)) => {
                        (heading.turn_right(), (x, y + distance))
                    }
                }
            });

    Ok(x.abs() + y.abs())
}

fn part2(instrs: &[Instruction]) -> Result<i16> {
    let positions = instrs.iter().scan((Heading::North, (0, 0)), |state, next| {
        let (heading, (x, y)) = *state;

        let (next_heading, next_coords) = match (heading, *next) {
            (Heading::North, Instruction::Left(distance)) => {
                (heading.turn_left(), (x - distance, y))
            }
            (Heading::North, Instruction::Right(distance)) => {
                (heading.turn_right(), (x + distance, y))
            }
            (Heading::East, Instruction::Left(distance)) => {
                (heading.turn_left(), (x, y + distance))
            }
            (Heading::East, Instruction::Right(distance)) => {
                (heading.turn_right(), (x, y - distance))
            }
            (Heading::South, Instruction::Left(distance)) => {
                (heading.turn_left(), (x + distance, y))
            }
            (Heading::South, Instruction::Right(distance)) => {
                (heading.turn_right(), (x - distance, y))
            }
            (Heading::West, Instruction::Left(distance)) => {
                (heading.turn_left(), (x, y - distance))
            }
            (Heading::West, Instruction::Right(distance)) => {
                (heading.turn_right(), (x, y + distance))
            }
        };

        *state = (next_heading, next_coords);
        Some(((x, y), next_coords))
    });

    let mut visited = HashSet::new();

    for ((mut start_x, mut start_y), (end_x, end_y)) in positions {
        let delta_y = end_y.cmp(&start_y) as i16;
        let delta_x = end_x.cmp(&start_x) as i16;

        while start_x != end_x || start_y != end_y {
            if !visited.insert((start_x, start_y)) {
                return Ok(start_x.abs() + start_y.abs());
            }

            start_x += delta_x;
            start_y += delta_y;
        }
    }
    Err(eyre!("No coordinates visited twice"))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2016, 1).open()?;
    let (instructions, parse_bench) =
        aoc_lib::bench(&ALLOC, "Parse", &|| Instruction::parse(&input))?;

    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| part1(&instructions))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| part2(&instructions))?;

    aoc_lib::display_results(
        "Day 1: No Time for a Taxicab",
        &[(&"", parse_bench), (&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

    Ok(())
}

#[cfg(test)]
mod tests_1601 {
    use super::*;

    #[test]
    fn part1_example() {
        let tests = [("R2, L3", 5), ("R2, R2, R2", 2), ("R5, L5, R5, R3", 12)];

        for &(test, expected) in &tests {
            let instructions = Instruction::parse(test).unwrap();
            assert_eq!(part1(&instructions).unwrap(), expected, "{}", test);
        }
    }

    #[test]
    fn part2_example() {
        let instructions = Instruction::parse("R8, R4, R4, R8").unwrap();

        let expected = 4;
        let actual = part2(&instructions).unwrap();

        assert_eq!(actual, expected);
    }
}
