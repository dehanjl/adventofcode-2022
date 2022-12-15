use adventofcode_2022::{is_real, runner};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::{cmp::Ordering, ops::RangeInclusive};

type Sensor = Loc;
type Beacon = Loc;
type MinMax = (i32, i32);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Loc {
    x: i32,
    y: i32,
}

impl Loc {
    fn manhattan(&self, other: &Loc) -> i32 {
        self.x_dist(other.x) + self.y_dist(other.y)
    }

    fn x_dist(&self, other_x: i32) -> i32 {
        (self.x - other_x).abs()
    }

    fn y_dist(&self, other_y: i32) -> i32 {
        (self.y - other_y).abs()
    }
}

/// A function that takes two inclusive ranges, and joins them if they overlap.
/// If they do not overlap, None is returned.
fn range_join(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
    match (a.start().cmp(b.end()), a.end().cmp(b.start())) {
        (Ordering::Greater, _) => None, // range a is after range b
        (_, Ordering::Less) => None,    // range a is before range b
        (_, _) => Some(*a.start().min(b.start())..=*a.end().max(b.end())), // there is some overlap, join them
    }
}

fn find_lims(sensors: &HashMap<Sensor, Beacon>) -> (MinMax, MinMax) {
    let x_min_max = sensors
        .keys()
        .chain(sensors.values())
        .map(|l| l.x)
        .minmax()
        .into_option()
        .unwrap();

    let y_min_max = sensors
        .keys()
        .chain(sensors.values())
        .map(|l| l.y)
        .minmax()
        .into_option()
        .unwrap();

    (x_min_max, y_min_max)
}

fn parse_input(input: &str) -> (HashMap<Sensor, Beacon>, HashSet<Beacon>) {
    let sensors = input
        .lines()
        .map(|line| {
            let (sx, sy, bx, by) = scan_fmt!(
                line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                i32,
                i32,
                i32,
                i32
            )
            .unwrap();

            (Loc { x: sx, y: sy }, Loc { x: bx, y: by })
        })
        .collect::<HashMap<_, _>>();

    let beacons = sensors
        .values()
        .map(|l| l.to_owned())
        .collect::<HashSet<_>>();

    (sensors, beacons)
}

fn compress_ranges(ranges: &mut Vec<RangeInclusive<i32>>) {
    let mut join_count = -1;
    while join_count != 0 {
        join_count = 0;
        loop {
            let a = ranges.remove(0); // take out a
            let rc = ranges.clone();
            let f = rc.iter().find_position(|b| range_join(&a, b).is_some());

            match f {
                Some((u, b)) => {
                    ranges.remove(u); // take out b
                    ranges.push(range_join(&a, b).unwrap()); // put in the new range
                    join_count += 1;
                }
                None => {
                    ranges.push(a); // found no ranges that could be joined with a, put it back
                    break;
                }
            }
        }
    }
}

fn part1(input: &str) {
    let target_row = if !is_real() { 10 } else { 2_000_000 };
    let (sensors, beacons) = parse_input(input);

    let mut ranges: Vec<RangeInclusive<i32>> = vec![];
    for (s, b) in sensors {
        let offset = s.manhattan(&b) - s.y_dist(target_row);

        if offset <= 0 {
            continue; // sensor range does not reach target row
        }

        let (x_start, x_stop) = (s.x - offset, s.x + offset);

        ranges.push(x_start..=x_stop);
        println!("{:?}", ranges);
    }

    compress_ranges(&mut ranges);
    println!("{:?}", ranges);

    let mut res = ranges
        .iter()
        .map(|r| (r.end() - r.start()).abs() + 1)
        .sum::<i32>();

    res -= beacons.iter().filter(|&&b| b.y == target_row).count() as i32;

    println!("Day 15 Part 1: {}", res);
}

fn part2(input: &str) {}

fn main() {
    runner(part1)
}
