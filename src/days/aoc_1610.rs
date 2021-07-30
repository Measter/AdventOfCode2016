use std::collections::{HashMap, VecDeque};

use aoc_lib::{day, parsers::unsigned_number, Bench, BenchResult, UserError};
use color_eyre::{eyre::eyre, Result};
use nom::{branch::alt, bytes::complete::tag, sequence::tuple};

day! {
    day 10: "Balance Bots"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let instrs: VecDeque<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| {
        let network = Network::with_instructions(instrs.clone());
        part1(network).map(Into::into)
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let instrs: VecDeque<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| {
        let network = Network::with_instructions(instrs.clone());
        part2(network).map(Into::into)
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TransferDest {
    Bot(usize),
    Output(usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Init {
        value: usize,
        bot: usize,
    },
    Transfer {
        src_bot: usize,
        low: TransferDest,
        high: TransferDest,
    },
}

impl Instruction {
    fn parse(input: &str) -> Result<Instruction> {
        match input.split_once(" ") {
            Some(("value", input)) => {
                let (_, (value, _, bot_id)) = tuple((
                    unsigned_number::<usize>,
                    tag(" goes to bot "),
                    unsigned_number::<usize>,
                ))(input)
                .map_err(|e| eyre!("Error parsing input: {}", e))?;

                Ok(Instruction::Init {
                    value: value?,
                    bot: bot_id?,
                })
            }
            Some(("bot", input)) => {
                let (_, (src_id, _, low_type, _, low_id, _, high_type, _, high_id)) =
                    tuple((
                        unsigned_number::<usize>,
                        tag(" gives low to "),
                        alt((tag("bot"), tag("output"))),
                        tag(" "),
                        unsigned_number::<usize>,
                        tag(" and high to "),
                        alt((tag("bot"), tag("output"))),
                        tag(" "),
                        unsigned_number::<usize>,
                    ))(input)
                    .map_err(|e| eyre!("Error parsing input: {}", e))?;

                let parse_dest = |t, v| match t {
                    "bot" => Ok(TransferDest::Bot(v)),
                    "output" => Ok(TransferDest::Output(v)),
                    _ => Err(eyre!("Invalid destination type: {}", t)),
                };

                Ok(Instruction::Transfer {
                    src_bot: src_id?,
                    low: parse_dest(low_type, low_id?)?,
                    high: parse_dest(high_type, high_id?)?,
                })
            }

            _ => Err(eyre!("Invalid instruction: {}", input)),
        }
    }
}

struct Network {
    bots: HashMap<usize, [Option<usize>; 2]>,
    outputs: HashMap<usize, usize>,
    instructions: VecDeque<Instruction>,
}

impl Network {
    fn with_instructions(instructions: VecDeque<Instruction>) -> Self {
        Self {
            bots: Default::default(),
            outputs: Default::default(),
            instructions,
        }
    }

    fn is_valid_bot(&self, id: usize) -> bool {
        !matches!(self.bots.get(&id), Some(bot) if bot.iter().all(|i| i.is_some()))
    }

    fn step(&mut self) -> Result<bool> {
        let next_instr = match self.instructions.pop_front() {
            Some(instr) => instr,
            None => return Ok(false),
        };

        match next_instr {
            Instruction::Init { value, bot: bot_id } => {
                let bot = self.bots.entry(bot_id).or_insert_with(Default::default);

                match bot {
                    [dst @ None, _] | [Some(_), dst @ None] => {
                        *dst = Some(value);
                        Ok(true)
                    }
                    [Some(_), Some(_)] => Err(eyre!(
                        "Bot {} already had two values for instruction \"{:?}\"",
                        bot_id,
                        next_instr
                    )),
                }
            }
            Instruction::Transfer {
                src_bot,
                low: low_dst,
                high: high_dst,
            } => {
                let bot = *self.bots.entry(src_bot).or_insert_with(Default::default);

                let [low, high] = if let [Some(left), Some(right)] = bot {
                    [left.min(right), right.max(left)]
                } else {
                    // Bots need two values to transfer. If it doesn't we wait until we do have two.
                    self.instructions.push_back(next_instr);
                    return Ok(true);
                };

                // Need to check if both destinations, if bots, have available space.
                for dst in [low_dst, high_dst] {
                    match dst {
                        TransferDest::Bot(dst_id) if !self.is_valid_bot(dst_id) => {
                            self.instructions.push_back(next_instr);
                            return Ok(true);
                        }
                        _ => {}
                    }
                }

                // Now we know the distinations are valid, *now* to perform the transfer.
                for (val, dst) in [(low, low_dst), (high, high_dst)] {
                    match dst {
                        TransferDest::Bot(dst_id) => {
                            let dst_bot = self.bots.entry(dst_id).or_insert_with(Default::default);
                            match dst_bot {
                                [dst @ None, _] | [Some(_), dst @ None] => *dst = Some(val),
                                _ => unreachable!(),
                            }
                        }
                        TransferDest::Output(dst_id) => {
                            self.outputs.insert(dst_id, val);
                        }
                    }
                }

                Ok(true)
            }
        }
    }
}

fn part1(mut network: Network) -> Result<usize> {
    while network.step()? {
        for bot in &network.bots {
            if let [Some(61), Some(17)] | [Some(17), Some(61)] = bot.1 {
                return Ok(*bot.0);
            }
        }
    }

    Err(eyre!("Unable to find bot"))
}

fn part2(mut network: Network) -> Result<usize> {
    while network.step()? {}

    (0..=2)
        .map(|i| network.outputs.get(&i).copied())
        .map(|i| i.ok_or_else(|| eyre!("Output not found")))
        .product()
}

#[cfg(test)]
mod tests_1610 {
    use aoc_lib::input;
    use maplit::hashmap;

    use super::*;

    #[test]
    fn parse_test() {
        let tests = [
            (
                "value 5 goes to bot 2",
                Instruction::Init { value: 5, bot: 2 },
            ),
            (
                "bot 2 gives low to bot 1 and high to bot 0",
                Instruction::Transfer {
                    src_bot: 2,
                    low: TransferDest::Bot(1),
                    high: TransferDest::Bot(0),
                },
            ),
            (
                "bot 1 gives low to output 1 and high to bot 0",
                Instruction::Transfer {
                    src_bot: 1,
                    low: TransferDest::Output(1),
                    high: TransferDest::Bot(0),
                },
            ),
        ];

        for (i, (test, expected)) in tests.iter().enumerate() {
            let actual = Instruction::parse(test).unwrap();
            assert_eq!(*expected, actual, "Failed: {}", i);
        }
    }

    #[test]
    fn part1_example() {
        let input = input(2016, 10)
            .example(aoc_lib::Example::Part1, 1)
            .open()
            .unwrap();
        let instrs: VecDeque<_> = input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let mut network = Network::with_instructions(instrs);
        let expected_outputs = hashmap! {
            0 => 5,
            1 => 2,
            2 => 3
        };

        while network.step().unwrap() {}

        assert_eq!(network.outputs, expected_outputs);
    }
}
