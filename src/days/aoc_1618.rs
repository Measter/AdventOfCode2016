use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 18,
    name: "Like a Rogue",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let (width, data) = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve(width, data, 40)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let (width, data) = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve(width, data, 400_000)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse(input: &str) -> Result<(usize, u128)> {
    let input = input.trim();
    let len = input.len();
    assert!(len < 126);

    let mut ret = 0;

    for ch in input.bytes() {
        let bit = ch == b'^';
        ret = (ret << 1) | bit as u128;
    }

    ret <<= 1;

    Ok((len, ret))
}

fn solve(width: usize, mut row: u128, num_rows: u32) -> u32 {
    let safe_mask = (u128::MAX << (width + 1)) | 1;
    let mut num_safe = 0;

    for _ in 0..num_rows {
        num_safe += (row | safe_mask).count_zeros();
        row = ((row >> 1) ^ (row << 1)) & !safe_mask;
    }

    num_safe
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let (width, start) = parse(&data).unwrap();
        let expected = 38;
        let actual = solve(width, start, 10);

        assert_eq!(expected, actual);
    }
}
