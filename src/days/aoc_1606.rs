use std::{array::IntoIter, cmp::Ordering};

use aoc_lib::{day, Bench, BenchResult, NoError};

day! {
    day 6: "Signals and Noise"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let input_lines: Vec<_> = input.lines().collect();
    b.bench(|| Ok::<_, NoError>(part(&input_lines, |f| f)).map(Into::into))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let input_lines: Vec<_> = input.lines().collect();
    b.bench(|| Ok::<_, NoError>(part(&input_lines, Ordering::reverse)).map(Into::into))
}

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
