use aoc_lib::{parsers::unsigned_number, TracingAlloc};
use color_eyre::Result;
use nom::{
    bytes::complete::tag,
    sequence::{preceded, separated_pair, terminated},
};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

// Implements the algorithm in this post: https://www.reddit.com/r/adventofcode/comments/5hbygy/2016_day_9_solutions/dazentu/
fn part1(input: &str) -> usize {
    let mut final_length = 0;
    let mut chars = input.char_indices();
    while let Some((idx, _)) = chars.next() {
        let chunk_parts = terminated(
            preceded(
                tag("("),
                separated_pair(unsigned_number::<usize>, tag("x"), unsigned_number::<usize>),
            ),
            tag(")"),
        )(&input[idx..]);

        if let Ok((_, (Ok(amt), Ok(rep)))) = chunk_parts {
            // Discard the chars from the iterator.
            (&mut chars).take_while(|(_, c)| *c != ')').for_each(|_| ());
            (&mut chars).take(amt).for_each(|_| ());

            final_length += amt * rep;
        } else {
            // No repetition, just add the weight of the current character to the final length.
            final_length += 1;
        }
    }

    final_length
}

fn part2(input: &str) -> usize {
    let mut final_length = 0;
    let mut weights = vec![1; input.len()];

    let mut chars = input.char_indices();
    while let Some((idx, _)) = chars.next() {
        let chunk_parts = terminated(
            preceded(
                tag("("),
                separated_pair(unsigned_number::<usize>, tag("x"), unsigned_number::<usize>),
            ),
            tag(")"),
        )(&input[idx..]);

        if let Ok((_, (Ok(amt), Ok(rep)))) = chunk_parts {
            // Discard the chars from the iterator.
            (&mut chars).take_while(|(_, c)| *c != ')').for_each(|_| ());

            // Adjust the weight of the following `amt` characters`.
            chars
                .clone()
                .take(amt)
                .for_each(|(idx, _)| weights[idx] *= rep);
        } else {
            // No repetition, just add the weight of the current character to the final length.
            final_length += weights[idx];
        }
    }

    final_length
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2016, 9).open()?;

    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| {
        let result = part1(&input.trim());
        Ok::<_, ()>(result)
    })?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| {
        let result = part2(&input.trim());
        Ok::<_, ()>(result)
    })?;

    aoc_lib::display_results(
        "Day 9: Explosives in Cyberspace",
        &[(&p1_res, p1_bench), (&p2_res, p2_bench)],
    );
    Ok(())
}

#[cfg(test)]
mod tests_1609 {
    use super::*;

    #[test]
    fn part1_test() {
        let inputs = aoc_lib::input(2016, 9)
            .example(aoc_lib::Example::Part1, 1)
            .open()
            .unwrap();

        for (input, expected) in inputs.lines().flat_map(|l| l.split_once(" | ")) {
            let expected = expected.parse::<usize>().unwrap();
            let actual = part1(input);

            assert_eq!(expected, actual, "{}", input);
        }
    }

    #[test]
    fn part2_test() {
        let inputs = aoc_lib::input(2016, 9)
            .example(aoc_lib::Example::Part2, 1)
            .open()
            .unwrap();

        for (input, expected) in inputs.lines().flat_map(|l| l.split_once(" | ")) {
            let expected = expected.parse::<usize>().unwrap();
            let actual = part2(input);

            assert_eq!(expected, actual, "{}", input);
        }
    }
}
