use aoc_lib::{day, Bench, BenchResult};

use std::fmt::Write;

day! {
    day 5: "How About a Nice Game of Chass?"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, u32>(part1(&input)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, u32>(part2(&input)))
}

fn part1(input: &str) -> String {
    let mut buf = input.to_owned();
    let mut password = String::new();

    for i in 0.. {
        buf.truncate(input.len());
        write!(&mut buf, "{}", i).unwrap();

        let hash = md5::compute(&buf);

        if let [0x00, 0x00, next @ 0x00..=0x0F, ..] = hash.0 {
            password.push(char::from_digit(next as _, 16).unwrap());
            if password.len() == 8 {
                break;
            }
        }
    }

    password
}

fn part2(input: &str) -> String {
    let mut buf = input.to_owned();
    let mut password = [None::<char>; 8];
    let mut found = 0;

    for i in 0.. {
        buf.truncate(input.len());
        write!(&mut buf, "{}", i).unwrap();

        let hash = md5::compute(&buf);

        match hash.0 {
            [0x00, 0x00, pos @ 0x00..=0x07, next, ..] if password[pos as usize].is_none() => {
                let next = char::from_digit((next >> 4) as _, 16).unwrap();
                password[pos as usize] = Some(next);
                found += 1;
                if found == 8 {
                    break;
                }
            }
            _ => {}
        }
    }

    password.iter().copied().map(Option::unwrap).collect()
}

#[cfg(test)]
mod tests_1605 {
    use super::*;

    // Commented out tests because they take bloody ages!

    // #[test]
    fn part1_test() {
        let door_id = "abc";
        let expected = "18f47a30";
        let actual = part1(door_id);

        assert_eq!(expected, actual);
    }

    // #[test]
    fn part2_test() {
        let door_id = "abc";
        let expected = "05ace8e3";
        let actual = part2(door_id);

        assert_eq!(expected, actual);
    }
}
