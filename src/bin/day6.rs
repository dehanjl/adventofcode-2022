use adventofcode_2022::runner;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

// TODO: Try to use bit shifting magic

fn part1(input: &str) {
    let chars = parse_input(input);

    for i in 0..chars.len() {
        let slice = &chars[i..i + 4];
        if slice.iter().unique().count() == 4 {
            println!("Part 1: {}", i + 4);
            return;
        }
    }
}

fn part2(input: &str) {
    let chars = parse_input(input);

    for i in 0..chars.len() {
        let slice = &chars[i..i + 14];
        if slice.iter().unique().count() == 14 {
            println!("Part 1: {}", i + 14);
            return;
        }
    }
}

fn main() {
    runner(part1);
    runner(part2);
}
