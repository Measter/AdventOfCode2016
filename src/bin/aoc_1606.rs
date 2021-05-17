use std::{array::IntoIter, cmp::Ordering};

use aoc_lib::TracingAlloc;
use color_eyre::Result;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn part(input: &[&str], f: fn(Ordering) -> Ordering) -> String {
    let mut output = vec!['!'; input[0].len()];

    for (i, b) in output.iter_mut().enumerate() {
        // As pure lazyness, I'll make use of the knowledge that the input is lowercase ASCII.
        let mut freq_array = [0u32; 26];
        for line in input {
            let idx = (line.as_bytes()[i] - b'a') as usize;
            freq_array[idx] += 1;
        }

        let most_common = IntoIter::new(freq_array)
            .enumerate()
            .filter(|(_, a)| *a > 0)
            .max_by(|(_, a), (_, b)| f(a.cmp(&b)))
            .map(|(i, _)| i)
            .unwrap();

        *b = (most_common as u8 + b'a') as char;
    }

    output.into_iter().collect()
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2016, 6).open()?;
    let input_lines: Vec<_> = input.lines().collect();
    let (p1_result, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| {
        Ok::<String, ()>(part(&input_lines, |f| f))
    })?;
    let (p2_result, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| {
        Ok::<String, ()>(part(&input_lines, Ordering::reverse))
    })?;

    aoc_lib::display_results(
        "Day 6: Signals and Noise",
        &[(&p1_result, p1_bench), (&p2_result, p2_bench)],
    );

    Ok(())
}

#[cfg(test)]
mod tests_1606 {
    use super::*;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(2016, 6)
            .example(aoc_lib::Example::Part1, 1)
            .open()
            .unwrap();
        let lines: Vec<_> = input.lines().collect();
        let expected = "easter";
        let actual = part(&lines, |f| f);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parto_test() {
        let input = aoc_lib::input(2016, 6)
            .example(aoc_lib::Example::Part1, 1)
            .open()
            .unwrap();
        let lines: Vec<_> = input.lines().collect();
        let expected = "advent";
        let actual = part(&lines, |f| f.reverse());

        assert_eq!(expected, actual);
    }
}
