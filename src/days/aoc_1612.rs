use aoc_lib::{day, Bench, BenchResult, NoError, UserError};

use color_eyre::eyre::{eyre, Result};

day! {
    day 12: "Leonardo's Monorail"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let instrs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| {
        let mut cpu = Cpu::default();
        cpu.execute(&instrs);

        Ok::<_, NoError>(cpu.registers[Register::A as usize])
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let instrs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| {
        let mut cpu = Cpu::default();
        cpu.registers[Register::C as usize] = 1;
        cpu.execute(&instrs);

        Ok::<_, NoError>(cpu.registers[Register::A as usize])
    })
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl Register {
    fn parse(r: &str) -> Option<Self> {
        match r {
            "a" | "A" => Some(Self::A),
            "b" | "B" => Some(Self::B),
            "c" | "C" => Some(Self::C),
            "d" | "D" => Some(Self::D),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Register(Register),
    Immediate(isize),
}

impl Value {
    fn parse(v: &str) -> Option<Self> {
        let as_num = v.parse();
        let as_reg = Register::parse(v);

        match (as_num, as_reg) {
            (Ok(v), _) => Some(Self::Immediate(v)),
            (_, Some(r)) => Some(Self::Register(r)),
            _ => None,
        }
    }

    fn get(self, cpu: &Cpu) -> isize {
        match self {
            Value::Register(reg) => cpu.registers[reg as usize],
            Value::Immediate(i) => i,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Copy { src: Value, dst: Register },
    Increment(Register),
    Decrement(Register),
    JumpNonZero { val: Value, offset: isize },
}

impl Instruction {
    fn parse(instr: &str) -> Result<Instruction> {
        let mut parts = instr.splitn(3, ' ');
        let op = parts.next();

        let par1 = parts.next().and_then(Value::parse);
        let par2 = parts.next().and_then(Value::parse);

        let instr = match (op, par1, par2) {
            (Some("cpy"), Some(src), Some(Value::Register(dst))) => Instruction::Copy { src, dst },
            (Some("jnz"), Some(val), Some(Value::Immediate(offset))) => {
                Instruction::JumpNonZero { val, offset }
            }
            (Some("inc"), Some(Value::Register(r)), None) => Instruction::Increment(r),
            (Some("dec"), Some(Value::Register(r)), None) => Instruction::Decrement(r),
            _ => return Err(eyre!("Invalid instruction: {}", instr)),
        };

        Ok(instr)
    }
}

#[derive(Debug, Default)]
struct Cpu {
    registers: [isize; 4],
}

impl Cpu {
    fn execute(&mut self, instrs: &[Instruction]) {
        let mut pc = 0;

        loop {
            let instr = match instrs.get(pc) {
                Some(i) => *i,
                None => return,
            };

            match instr {
                Instruction::Copy { src, dst } => {
                    self.registers[dst as usize] = src.get(self);
                    pc += 1;
                }
                Instruction::Increment(reg) => {
                    self.registers[reg as usize] += 1;
                    pc += 1;
                }
                Instruction::Decrement(reg) => {
                    self.registers[reg as usize] -= 1;
                    pc += 1;
                }
                Instruction::JumpNonZero { val, offset } => {
                    let tst_val = val.get(self);
                    if tst_val != 0 {
                        pc = pc.wrapping_add(offset as usize); // Just let overflow handle the negative case.
                    } else {
                        pc += 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_1612 {
    use aoc_lib::input;

    use super::*;

    #[test]
    fn part1() {
        let instrs = input(12)
            .example(aoc_lib::Example::Part1, 1)
            .open()
            .unwrap();

        let instrs: Vec<_> = instrs
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let mut cpu = Cpu::default();
        cpu.execute(&instrs);

        assert_eq!(cpu.registers[Register::A as usize], 42);
    }
}
