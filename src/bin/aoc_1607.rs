use aoc_lib::{misc::ArrWindows, TracingAlloc};
use color_eyre::{eyre::eyre, Report, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2016, 7).open()?;
    let input_lines: Vec<_> = input.lines().collect();

    let (p1_result, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| {
        Ok::<usize, ()>(input_lines.iter().filter(|l| part1(l)).count())
    })?;

    let (p2_result, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| {
        let mut count = 0;
        for address in &input_lines {
            if part2(address)? {
                count += 1;
            }
        }

        Ok::<usize, Report>(count)
    })?;

    aoc_lib::display_results(
        "Day 7: Internet Protocol Version 7",
        &[(&p1_result, p1_bench), (&p2_result, p2_bench)],
    );

    Ok(())
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
