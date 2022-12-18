use adventofcode_2022::runner;
use hashbrown::HashSet;
use itertools::Itertools;
use scan_fmt::scan_fmt;

type Loc = (i32, i32, i32);

fn sides((x, y, z): (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn parse_input(input: &str) -> HashSet<Loc> {
    input
        .lines()
        .map(|line| scan_fmt!(line, "{},{},{}", i32, i32, i32).unwrap())
        .collect()
}

fn part1(input: &str) {
    let coords = parse_input(input);

    let surface_area = coords
        .iter()
        .flat_map(|&p| sides(p))
        .filter(|s| !coords.contains(s))
        .count();

    println!("Day 18 Part 1: {}", surface_area);
}

fn part2(input: &str) {}

fn main() {
    runner(part1);
}
