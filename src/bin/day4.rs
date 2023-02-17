use std::cmp::Ordering;

use adventofcode_2022::runner;
use scan_fmt::scan_fmt;

fn parse_input(input: &str) -> Vec<(u32, u32, u32, u32)> {
    input
        .lines()
        .flat_map(|line| scan_fmt!(line, "{d}-{d},{d}-{d}", u32, u32, u32, u32))
        .collect()
}

fn part1(input: &str) {
    let sum = parse_input(input)
        .iter()
        .map(|(r1, f1, r2, f2)| (r1.cmp(r2), f1.cmp(f2)))
        .filter(|(x, y)| {
            [x, y].iter().any(|&x| x == &Ordering::Equal)
                || matches!(
                    (x, y),
                    (Ordering::Less, Ordering::Greater) | (Ordering::Greater, Ordering::Less)
                )
        })
        .count();

    println!("Day 4 Part 1: {sum}");
}

fn part2(input: &str) {
    let sum = parse_input(input)
        .iter()
        .map(|(r1, f1, r2, f2)| (r1.cmp(r2), f1.cmp(f2), r1.cmp(f2), f1.cmp(r2)))
        .filter(|(a, b, c, d)| {
            [a, b, c, d].iter().any(|&x| x == &Ordering::Equal)
                || matches!(
                    (a, b, c, d),
                    (Ordering::Less, Ordering::Greater, _, _) // range 1 contains range 2
                        | (Ordering::Greater, Ordering::Less, _, _) // range 2 contains range 1
                        | (_, Ordering::Less, _, Ordering::Greater) // range 1 end is in range 2
                        | (Ordering::Greater, _, Ordering::Less, _) // range 1 start is in range 2
                )
        })
        .count();

    println!("Day 4 Part 2: {sum}");
}

fn main() {
    runner(part1);
    runner(part2);
}
