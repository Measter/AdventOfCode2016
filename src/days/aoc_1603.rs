use aoc_lib::{day, Bench, BenchError, BenchResult};
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

day! {
    day 3: "Squares With Three Sides"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let tris = parse_p1(input).map_err(|e| BenchError::UserError(e.into()))?;
    b.bench(|| valid_tris(&tris).map(Into::into))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let tris = parse_p2(input).map_err(|e| BenchError::UserError(e.into()))?;
    b.bench(|| valid_tris(&tris).map(Into::into))
}

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
