use aoc_lib::{Bench, BenchError, BenchResult, Day};
use color_eyre::{eyre::eyre, Result};

pub const DAY: Day = Day {
    day: 8,
    name: "Two-Factor Authentication",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let instrs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(|e| BenchError::UserError(e.into()))?;

    b.bench(|| {
        let mut display = Display::new(50, 6);
        for &instr in &instrs {
            display
                .execute(instr)
                .map_err(|e| BenchError::UserError(e.into()))?;
        }

        Ok::<_, BenchError>(display.num_lit())
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let instrs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(|e| BenchError::UserError(e.into()))?;

    b.bench_alt(|| {
        let mut display = Display::new(50, 6);
        for &instr in &instrs {
            display
                .execute(instr)
                .map_err(|e| BenchError::UserError(e.into()))?;
        }

        Ok::<_, BenchError>(display.render_display())
    })
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    Rect { x: usize, y: usize },
    RotateRow { y: usize, amount: usize },
    RotateCol { x: usize, amount: usize },
}

impl Instruction {
    fn parse(line: &str) -> Result<Self> {
        let err_msg = || eyre!("Invalid instruction: {:?}", line);
        let line = line.trim();

        let res = if line.starts_with("rect") {
            let (x, y) = line
                .trim_start_matches("rect ")
                .split_once('x')
                .ok_or_else(err_msg)?;
            Instruction::Rect {
                x: x.parse().map_err(|_| err_msg())?,
                y: y.parse().map_err(|_| err_msg())?,
            }
        } else if line.starts_with("rotate row y=") {
            let (y, amount) = line
                .trim_start_matches("rotate row y=")
                .split_once(" by ")
                .ok_or_else(err_msg)?;

            Instruction::RotateRow {
                y: y.parse().map_err(|_| err_msg())?,
                amount: amount.parse().map_err(|_| err_msg())?,
            }
        } else if line.starts_with("rotate column x=") {
            let (x, amount) = line
                .trim_start_matches("rotate column x=")
                .split_once(" by ")
                .ok_or_else(err_msg)?;
            Instruction::RotateCol {
                x: x.parse().map_err(|_| err_msg())?,
                amount: amount.parse().map_err(|_| err_msg())?,
            }
        } else {
            return Err(err_msg());
        };

        Ok(res)
    }
}

struct Display {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
    col_rot_buf: Vec<bool>,
}

impl Display {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![false; width * height],
            col_rot_buf: vec![false; height],
        }
    }

    fn execute(&mut self, instr: Instruction) -> Result<()> {
        match instr {
            Instruction::Rect { x, y } => {
                if x >= self.width || y >= self.height {
                    return Err(eyre!("Invalid rect dimensions for screen: {}x{}", x, y));
                }

                self.pixels
                    .chunks_exact_mut(self.width)
                    .take(y)
                    .for_each(|r| r[..x].fill(true));
            }
            Instruction::RotateRow { y, amount } => {
                if y >= self.height {
                    return Err(eyre!("Invalid row {}", y));
                }

                let start = y * self.width;
                let end = start + self.width;
                self.pixels[start..end].rotate_right(amount);
            }
            Instruction::RotateCol { x, amount } => {
                if x >= self.width {
                    return Err(eyre!("Invalid col: {}", x));
                }

                // This is really dumb, but makes the rotation easier, so bugger it.
                self.pixels
                    .chunks_exact(self.width)
                    .zip(&mut self.col_rot_buf)
                    .for_each(|(src, dst)| *dst = src[x]);

                self.col_rot_buf.rotate_right(amount);

                self.pixels
                    .chunks_exact_mut(self.width)
                    .zip(&self.col_rot_buf)
                    .for_each(|(dst, src)| dst[x] = *src);
            }
        }

        Ok(())
    }

    fn num_lit(&self) -> usize {
        self.pixels.iter().filter(|p| **p).count()
    }

    fn render_display(&self) -> String {
        let mut buf = String::with_capacity(self.width);

        for row in self.pixels.chunks_exact(self.width) {
            buf.extend(row.iter().map(|&p| if p { '#' } else { ' ' }));
            buf.push('\n');
        }

        buf
    }
}

#[cfg(test)]
mod tests_1608 {
    use super::*;

    #[test]
    fn parse() {
        let input = aoc_lib::input(8)
            .example(aoc_lib::Example::Parse, 1)
            .open()
            .unwrap();
        let expected = [
            Instruction::Rect { x: 1, y: 2 },
            Instruction::RotateRow { y: 1, amount: 2 },
            Instruction::RotateCol { x: 3, amount: 1 },
        ];

        for (line, expected) in input.lines().zip(expected.iter()) {
            let actual = Instruction::parse(line).unwrap();
            assert_eq!(actual, *expected)
        }
    }

    #[test]
    fn part1() {
        let input = aoc_lib::input(8)
            .example(aoc_lib::Example::Part1, 1)
            .open()
            .unwrap();
        let instructions = input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .map(Result::unwrap);

        #[rustfmt::skip]
        let actual = [
            [
                true,  true,  true,  false, false, false, false,
                true,  true,  true,  false, false, false, false,
                false, false, false, false, false, false, false,
            ],
            [
                true,  false, true,  false, false, false, false,
                true,  true,  true,  false, false, false, false,
                false, true,  false, false, false, false, false,
            ],
            [
                false, false, false, false, true,  false, true,
                true,  true,  true,  false, false, false, false,
                false, true,  false, false, false, false, false,
            ],
            [
                false, true,  false, false, true,  false, true,
                true,  false, true,  false, false, false, false,
                false, true,  false, false, false, false, false,
            ]
        ];

        let mut display = Display::new(7, 3);

        for (instr, actual) in instructions.zip(actual) {
            display.execute(instr).unwrap();
            assert_eq!(display.pixels, actual.as_ref(), "{:?}", instr);
        }
    }
}
