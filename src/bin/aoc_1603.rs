#![allow(clippy::unnecessary_wraps)]

use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn parse_p1(input: &str) -> Result<Vec<[u16; 3]>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().map(str::parse);
            let ((a, b), c) = parts
                .next()
                .zip(parts.next())
                .zip(parts.next())
                .ok_or_else(|| eyre!("Expected 3 numbers"))?;

            Ok([a?, b?, c?])
        })
        .collect()
}

fn parse_p2(input: &str) -> Result<Vec<[u16; 3]>> {
    input
        .lines()
        .map(|line| -> Result<_> {
            let mut parts = line.split_whitespace().map(str::parse);
            let ((a, b), c) = parts
                .next()
                .zip(parts.next())
                .zip(parts.next())
                .ok_or_else(|| eyre!("Expected 3 numbers"))?;

            Ok([a?, b?, c?])
        })
        .tuples()
        .map(|(a, b, c)| -> Result<_> {
            let [a1, b1, c1] = a?;
            let [a2, b2, c2] = b?;
            let [a3, b3, c3] = c?;
            Ok([[a1, a2, a3], [b1, b2, b3], [c1, c2, c3]])
        })
        .try_fold(Vec::new(), |mut acc, trio| {
            acc.extend(trio?.as_ref());
            Ok(acc)
        })
}

fn valid_tris(tris: &[[u16; 3]]) -> Result<usize> {
    Ok(tris
        .iter()
        .filter(|[a, b, c]| a + b > *c && a + c > *b && b + c > *a)
        .count())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2016, 3).open()?;
    let (p1_tris, parse_p1_bench) = aoc_lib::bench(&ALLOC, "Parse P1", &|| parse_p1(&input))?;
    let (p2_tris, parse_p2_bench) = aoc_lib::bench(&ALLOC, "Parse P2", &|| parse_p2(&input))?;

    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| valid_tris(&p1_tris))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| valid_tris(&p2_tris))?;

    aoc_lib::display_results(
        "Day 3: Squares With Three Sides",
        &[
            (&"", parse_p1_bench),
            (&"", parse_p2_bench),
            (&p1_res, p1_bench),
            (&p2_res, p2_bench),
        ],
    );

    Ok(())
}

#[cfg(test)]
mod tests_1603 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_test() {
        let expected = 0;
        let actual = valid_tris(&[[5, 10, 25]]).unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_p2_test() {
        let input = aoc_lib::input(2016, 3)
            .example(Example::Parse, 1)
            .open()
            .unwrap();
        let expected = vec![
            [101, 102, 103],
            [301, 302, 303],
            [501, 502, 503],
            [201, 202, 203],
            [401, 402, 403],
            [601, 602, 603],
        ];

        let actual = parse_p2(&input).unwrap();

        assert_eq!(expected, actual);
    }
}
