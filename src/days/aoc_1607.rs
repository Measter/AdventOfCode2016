use aoc_lib::{day, misc::ArrWindows, Bench, BenchError, BenchResult};
use color_eyre::{eyre::eyre, Result};

day! {
    day 7: "Internet Protocal Version 7"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let input_lines: Vec<_> = input.lines().collect();
    b.bench(|| Ok::<_, u32>(input_lines.iter().filter(|l| part1(l)).count()))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    let input_lines: Vec<_> = input.lines().collect();
    b.bench(|| {
        let mut count = 0;
        for address in &input_lines {
            if part2(address).map_err(|e| BenchError::UserError(e.into()))? {
                count += 1;
            }
        }

        Ok::<usize, BenchError>(count)
    })
}

fn part1(address: &str) -> bool {
    let mut is_in_brackets = false;
    let mut found_match = false;

    for &[a, b, c, d] in ArrWindows::new(address.as_bytes()) {
        if d == b'[' {
            is_in_brackets = true;
            continue;
        }

        if is_in_brackets && d == b']' {
            is_in_brackets = false;
            continue;
        }

        if is_in_brackets && a == d && b == c && a != b {
            return false;
        }

        found_match |= a == d && b == c && a != b;
    }

    found_match
}

fn part2(address: &str) -> Result<bool> {
    // Time to break out the vecs!
    let mut supernets = Vec::new();
    let mut hypernets = Vec::new();

    let mut remaining = address;
    while let Some((supernet, rem)) = remaining.split_once('[') {
        let (hypernet, rem) = rem
            .split_once(']')
            .ok_or_else(|| eyre!("Invalid address"))?;

        remaining = rem;
        supernets.push(supernet);
        hypernets.push(hypernet);
    }

    if !remaining.is_empty() {
        supernets.push(remaining);
    }

    for supernet in supernets {
        for &[a, b, _] in ArrWindows::new(supernet.as_bytes()).filter(|[a, _, c]| a == c) {
            let matching = hypernets
                .iter()
                .any(|hn| ArrWindows::new(hn.as_bytes()).any(|&hn| hn == [b, a, b]));

            if matching {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests_1607 {
    use super::*;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(2016, 7)
            .example(aoc_lib::Example::Part1, 1)
            .open()
            .unwrap();

        let expected = [true, false, false, true];

        for (test, expected) in input.lines().zip(expected.iter()) {
            let actual = part1(test);
            assert_eq!(*expected, actual, "{}", test);
        }
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(2016, 7)
            .example(aoc_lib::Example::Part2, 1)
            .open()
            .unwrap();

        let expected = [true, false, true, true];

        for (test, expected) in input.lines().zip(expected.iter()) {
            let actual = part2(test).unwrap();
            assert_eq!(*expected, actual, "{}", test);
        }
    }
}
