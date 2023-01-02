use aoc_lib::{misc::ArrChunks, Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 16,
    name: "Dragon Checksum",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve(&data, 272)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve(&data, 35_651_584)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse(input: &str) -> Result<Vec<u8>> {
    Ok(input.trim().bytes().map(|b| b - b'0').collect())
}

fn solve(data: &[u8], needed_len: usize) -> String {
    let mut data = data.to_owned();
    let mut buffer = Vec::new();

    while data.len() < needed_len {
        buffer.extend_from_slice(&data);
        buffer.push(0);
        buffer.extend(data.drain(..).rev().map(|b| !b & 1));

        std::mem::swap(&mut data, &mut buffer);
    }

    data.truncate(needed_len);

    while data.len() % 2 == 0 {
        for [a, b] in ArrChunks::new(&data) {
            buffer.push(!(a ^ b) & 1);
        }

        data.clear();
        std::mem::swap(&mut data, &mut buffer);
    }

    data.into_iter().map(|b| (b + b'0') as char).collect()
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

        let (data, rest) = data.split_once(' ').unwrap();
        let (len, expected) = rest.split_once(' ').unwrap();

        let data = parse(data).unwrap();
        let len = len.parse().unwrap();

        let actual = solve(&data, len);

        assert_eq!(expected, actual);
    }
}
