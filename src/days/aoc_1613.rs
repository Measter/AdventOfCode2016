use std::{
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    ops::Sub,
};

use aoc_lib::{Bench, BenchResult, Day, NoError, UserError};

pub const DAY: Day = Day {
    day: 13,
    name: "A Maze of Twisty Little Cubicles",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = input.parse().map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(data, Point::new(31, 39))))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = input.parse().map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(data)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn neighbours(self) -> [Point; 4] {
        let Point { x, y } = self;
        [
            Point { x, y: y - 1 },
            Point { x: x - 1, y },
            Point { x: x + 1, y },
            Point { x, y: y + 1 },
        ]
    }

    fn estimate_cost(self, target: Point) -> u16 {
        (self.x.sub(target.x).abs() + self.y.sub(target.y).abs()) as _
    }

    fn is_in_bounds(self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    fn is_wall(self, fav_num: i16) -> bool {
        let poly = self.x * self.x + 3 * self.x + 2 * self.x * self.y + self.y + self.y * self.y;
        let sum = poly + fav_num;
        sum.count_ones() & 1 == 1
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct State {
    hueristic_cost: u16,
    cost: u16,
    pos: Point,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        (self.hueristic_cost + self.cost) == (other.hueristic_cost + other.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.hueristic_cost + other.cost).cmp(&(self.hueristic_cost + self.cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(fav_num: i16, target: Point) -> u16 {
    let origin = Point::new(1, 1);
    let mut queue = BinaryHeap::new();
    let mut dist = HashMap::new();

    dist.insert(origin, 0);
    queue.push(State {
        hueristic_cost: 0,
        cost: 0,
        pos: origin,
    });

    while let Some(next) = queue.pop() {
        for neighbour in next.pos.neighbours() {
            if !neighbour.is_in_bounds() || neighbour.is_wall(fav_num) {
                continue;
            }

            let total_cost = next.cost + 1;
            if neighbour == target {
                return total_cost;
            }

            match dist.entry(neighbour) {
                Entry::Occupied(old_state) if total_cost > *old_state.get() => continue,
                Entry::Occupied(mut state) => {
                    state.insert(total_cost);
                }
                Entry::Vacant(state) => {
                    state.insert(total_cost);
                }
            };

            queue.push(State {
                hueristic_cost: neighbour.estimate_cost(target),
                cost: total_cost,
                pos: neighbour,
            });
        }
    }

    panic!("path not found");
}

fn part2(fav_num: i16) -> usize {
    let origin = Point::new(1, 1);
    let mut queue = BinaryHeap::new();
    let mut dist = HashMap::new();

    dist.insert(origin, 0);
    queue.push(State {
        hueristic_cost: 0,
        cost: 0,
        pos: origin,
    });

    while let Some(next) = queue.pop() {
        for neighbour in next.pos.neighbours() {
            if !neighbour.is_in_bounds() || neighbour.is_wall(fav_num) {
                continue;
            }

            let total_cost = next.cost + 1;
            if total_cost > 50 {
                continue;
            }

            match dist.entry(neighbour) {
                Entry::Occupied(old_state) if total_cost > *old_state.get() => continue,
                Entry::Occupied(mut state) => {
                    state.insert(total_cost);
                }
                Entry::Vacant(state) => {
                    state.insert(total_cost);
                }
            };

            queue.push(State {
                hueristic_cost: 0,
                cost: total_cost,
                pos: neighbour,
            });
        }
    }

    dist.len()
}

#[cfg(test)]
mod tests {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn generate_test() {
        let input = aoc_lib::input(13)
            .example(Example::Parse, 1)
            .open()
            .unwrap();

        let fav_num = 10;

        for (y, line) in input.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                let is_wall_tile = tile == '#';
                assert_eq!(
                    is_wall_tile,
                    Point::new(x as _, y as _).is_wall(fav_num,),
                    "x: {}, y: {}",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn part1_test() {
        let expected = 11;
        let actual = part1(10, Point::new(7, 4));
        assert_eq!(expected, actual);
    }
}
