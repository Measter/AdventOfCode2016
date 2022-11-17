use aoc_lib::{Bench, BenchError, BenchResult, Day, NoError};
use color_eyre::eyre::{eyre, Result};

pub const DAY: Day = Day {
    day: 4,
    name: "Security Through Obscurity",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let rooms: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Room::parse)
        .collect::<Result<_, _>>()
        .map_err(|e| BenchError::UserError(e.into()))?;

    b.bench(|| Ok::<u32, NoError>(rooms.iter().filter(|r| r.is_real()).map(|r| r.id).sum()))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let rooms: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Room::parse)
        .collect::<Result<_, _>>()
        .map_err(|e| BenchError::UserError(e.into()))?;

    b.bench(|| {
        let mut buf = String::new();
        for room in &rooms {
            buf.clear();
            room.decrypt_name(&mut buf);
            if buf == "northpole object storage" {
                return Ok(room.id);
            }
        }

        Err(eyre!("Error: room not found"))
    })
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Room<'a> {
    name: &'a str,
    id: u32,
    checksum: &'a str,
}

impl<'a> Room<'a> {
    fn parse(input: &str) -> Result<Room> {
        let (left, right) = input
            .split_once('[')
            .ok_or_else(|| eyre!("Invalid input"))?;
        let checksum = right.trim_end_matches(']');

        let (name, id) = left
            .rsplit_once('-')
            .ok_or_else(|| eyre!("Invalid input"))?;
        Ok(Room {
            name,
            checksum,
            id: id.parse()?,
        })
    }

    fn is_real(&self) -> bool {
        let mut letter_freq = [0; 26];

        self.name
            .chars()
            .filter(|c| c.is_ascii_lowercase())
            .map(|c| c as u8 - b'a')
            .for_each(|c| letter_freq[c as usize] += 1);

        let mut letters = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        letters.sort_by(|a, b| {
            let a_idx = (*a as u8 - b'a') as usize;
            let b_idx = (*b as u8 - b'a') as usize;
            let a_val = letter_freq[a_idx];
            let b_val = letter_freq[b_idx];

            b_val.cmp(&a_val)
        });

        self.checksum.chars().eq(letters[..5].iter().copied())
    }

    fn decrypt_name(&self, buf: &mut String) {
        let letters = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        buf.extend(self.name.chars().map(|c| match c {
            '-' => ' ',
            'a'..='z' => {
                let c_val = (c as u8 - b'a') as usize + self.id as usize;
                letters[c_val % 26]
            }
            _ => c,
        }));
    }
}

#[cfg(test)]
mod tests_1604 {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn parse_test() {
        let room = "aaaaa-bbb-z-y-x-123[abxyz]";
        let expected = Room {
            name: "aaaaa-bbb-z-y-x",
            id: 123,
            checksum: "abxyz",
        };

        let actual = Room::parse(room).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(4).example(Example::Parse, 1).open().unwrap();
        let rooms: Vec<_> = input
            .lines()
            .map(Room::parse)
            .collect::<Result<_>>()
            .unwrap();

        let expected_is_real = [true, true, true, false];
        let expected_sum = 1514;
        let mut actual_sum = 0;

        for (i, (room, expected_is_real)) in rooms.iter().zip(&expected_is_real).enumerate() {
            let actual_is_real = room.is_real();
            assert_eq!(actual_is_real, *expected_is_real, "{}", i);
            if actual_is_real {
                actual_sum += room.id;
            }
        }

        assert_eq!(actual_sum, expected_sum);
    }

    #[test]
    fn part2_test() {
        let input = Room {
            name: "qzmt-zixmtkozy-ivhz",
            id: 343,
            checksum: "",
        };

        let expected = "very encrypted name";
        let mut actual = String::new();
        input.decrypt_name(&mut actual);

        assert_eq!(expected, actual);
    }
}
