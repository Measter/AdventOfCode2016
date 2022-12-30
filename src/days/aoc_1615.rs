use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

pub const DAY: Day = Day {
    day: 15,
    name: "Timing is Everything",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(solve(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let mut data = parse(input).map_err(UserError)?;
    let num_discs = data.len() + 1;
    data.push(Disc {
        id: num_discs as _,
        start_pos: 0,
        num_posses: 11,
    });

    b.bench(|| Ok::<_, NoError>(solve(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
struct Disc {
    id: u32,
    start_pos: u32,
    num_posses: u32,
}

impl Disc {
    fn is_hole(self, time: u32) -> bool {
        (self.id + self.start_pos + time) % self.num_posses == 0
    }
}

fn parse(input: &str) -> Result<Vec<Disc>> {
    fn split_line(line: &str) -> Option<(&str, &str, &str)> {
        let line = line.strip_prefix("Disc #")?;
        let (id, line) = line.split_once(" has ")?;
        let (num_posses, line) = line.split_once(" positions; at time=0, it is at position ")?;
        let cur_pos = line.strip_suffix('.')?;
        Some((id, num_posses, cur_pos))
    }

    let mut discs = Vec::new();

    for line in input.lines() {
        let Some((id, num_posses, cur_pos)) = split_line(line) else {
            return Err(eyre!("Invalid disc definition"));
        };

        discs.push(Disc {
            id: id.parse()?,
            start_pos: cur_pos.parse()?,
            num_posses: num_posses.parse()?,
        })
    }

    Ok(discs)
}

fn solve(discs: &[Disc]) -> u32 {
    let mut cycle_len = discs[0].num_posses;
    let mut time = (0..).find(|t| discs[0].is_hole(*t)).unwrap();
    let mut remaining_discs = &discs[1..];

    loop {
        let mut start_idx = 0;
        for disc in remaining_discs {
            if disc.is_hole(time) {
                start_idx += 1;
                cycle_len *= disc.num_posses;
            } else {
                break;
            }
        }

        remaining_discs = &remaining_discs[start_idx..];

        if remaining_discs.is_empty() {
            return time;
        }
        time += cycle_len;
    }
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

        let discs = parse(&data).unwrap();
        let expected = 5;
        let actual = solve(&discs);

        assert_eq!(expected, actual);
    }
}
