use adventofcode_2022::{is_real, runner};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::{
    cmp::Ordering,
    i64::{MAX, MIN},
    ops::RangeInclusive,
};

type Sensor = Loc;
type Beacon = Loc;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Loc {
    x: i64,
    y: i64,
}

impl Loc {
    fn manhattan(&self, other: &Loc) -> i64 {
        self.x_dist(other.x) + self.y_dist(other.y)
    }

    fn x_dist(&self, other_x: i64) -> i64 {
        (self.x - other_x).abs()
    }

    fn y_dist(&self, other_y: i64) -> i64 {
        (self.y - other_y).abs()
    }
}

/// A function that takes two inclusive ranges, and joins them if they overlap.
/// If they do not overlap, None is returned.
fn range_join(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
    match (a.start().cmp(b.end()), a.end().cmp(b.start())) {
        (Ordering::Greater, _) => None, // range a is after range b
        (_, Ordering::Less) => None,    // range a is before range b
        (_, _) => Some(*a.start().min(b.start())..=*a.end().max(b.end())), // there is some overlap, join them
    }
}

fn parse_input(input: &str) -> (HashMap<Sensor, i64>, HashSet<Beacon>) {
    let mut beacons: HashSet<Beacon> = HashSet::new();
    let sensors = input
        .lines()
        .map(|line| {
            let (sx, sy, bx, by) = scan_fmt!(
                line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                i64,
                i64,
                i64,
                i64
            )
            .unwrap();

            let s = Loc { x: sx, y: sy };
            let b = Loc { x: bx, y: by };
            let d = s.manhattan(&b);

            beacons.insert(b);

            (s, d)
        })
        .collect::<HashMap<_, _>>();

    (sensors, beacons)
}

fn compress_ranges(ranges: &mut Vec<RangeInclusive<i64>>) {
    loop {
        if ranges.len() <= 1 {
            break; // break if there are no ranges to join
        }

        let mut altered = false;
        for _ in 0..ranges.len() {
            let a = ranges.remove(0); // take out a
            let rc = ranges.clone();
            let f = rc.iter().find_position(|b| range_join(&a, b).is_some());

            match f {
                Some((u, b)) => {
                    ranges.remove(u); // take out b
                    ranges.push(range_join(&a, b).unwrap()); // put in the new range
                    altered = true;
                    break;
                }
                None => {
                    ranges.push(a); // found no ranges that could be joined with a, put it back
                }
            }
        }

        if !altered {
            break; // break if we didn't join any ranges
        }
    }
}

/// Immutable variant of the above function. Is in fact significantly slower.
fn compress_ranges_v2(ranges: &Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    let mut ranges = ranges.to_vec();
    loop {
        if ranges.len() <= 1 {
            return ranges; // return if there are no ranges to join
        }

        let l = ranges.len();

        ranges = ranges.iter().fold(vec![], |acc, it| {
            if acc.len() < 1 {
                vec![it.clone()]
            } else {
                let k = acc.iter().find_position(|r| range_join(r, it).is_some());
                match k {
                    Some((u, _)) => acc
                        .iter()
                        .take(u)
                        .chain(acc.iter().skip(u + 1))
                        .chain(vec![range_join(&acc[u], it).unwrap()].iter())
                        .cloned()
                        .collect_vec(),
                    None => acc
                        .iter()
                        .chain(vec![it.clone()].iter())
                        .cloned()
                        .collect_vec(),
                }
            }
        });

        if ranges.len() == l {
            return ranges; // return if no ranges were joined
        }
    }
}

fn determine_ranges(
    sensors: &HashMap<Sensor, i64>,
    target_row: i64,
    mn: i64,
    mx: i64,
) -> Vec<RangeInclusive<i64>> {
    sensors
        .iter()
        .filter_map(|(s, d)| {
            let offset = d - s.y_dist(target_row);
            if offset > 0 {
                Some((s.x - offset).max(mn)..=(s.x + offset).min(mx))
            } else {
                None
            }
        })
        .collect()
}

fn part1(input: &str) {
    let target_row = if !is_real() { 10 } else { 2_000_000 };
    let (sensors, beacons) = parse_input(input);

    let mut ranges = determine_ranges(&sensors, target_row, MIN, MAX);
    compress_ranges(&mut ranges);

    let mut res = ranges
        .iter()
        .map(|r| (r.end() - r.start()).abs() + 1)
        .sum::<i64>();

    res -= beacons.iter().filter(|&&b| b.y == target_row).count() as i64;

    println!("Day 15 Part 1: {}", res);
}

fn part2(input: &str) {
    let (t_min, t_max) = (0, if !is_real() { 20 } else { 4_000_000 });
    let (sensors, _) = parse_input(input);

    // TODO: Invert this by iterating through sensors first
    let (y, _, r) = (t_min..=t_max)
        .map(|target_row| {
            let mut ranges = determine_ranges(&sensors, target_row, t_min, t_max);
            compress_ranges(&mut ranges);
            (target_row, ranges)
        })
        .map(|(row, ranges)| {
            let size = ranges
                .iter()
                .map(|r| (r.end() - r.start()).abs() + 1)
                .sum::<i64>();

            (row, size - 1, ranges)
        })
        .find(|(_, s, _)| *s < t_max)
        .unwrap();

    // TODO: Make this less bad
    let x = (t_min..=t_max)
        .find(|x| !r.iter().any(|r| r.contains(x)))
        .unwrap();

    let res = x * 4_000_000 + y;

    println!("Day 15 Part 2: {} (x={}, y={})", res, x, y);
}

fn main() {
    runner(part1);
    runner(part2);
}
