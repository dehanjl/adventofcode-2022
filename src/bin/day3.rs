use adventofcode_2022::runner;
use hashbrown::HashSet;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| ('a'..='z').chain('A'..='Z').position(|x| x == c).unwrap() as u32 + 1)
                .collect()
        })
        .collect()
}

fn part1(input: &str) {
    let res: u32 = parse_input(input)
        .iter()
        .map(|v| v.split_at(v.len() / 2))
        .map(|t| {
            (
                t.0.iter().cloned().collect::<HashSet<_>>(),
                t.1.iter().cloned().collect::<HashSet<_>>(),
            )
        })
        .fold(0, |acc, (x, y)| acc + x.intersection(&y).sum::<u32>());

    println!("Day 3 Part 1: {res}");
}

fn part2(input: &str) {
    let res = parse_input(input)
        .iter()
        .map(|v| HashSet::from_iter(v.iter().cloned()))
        .into_iter()
        .tuples()
        .fold(0, |acc, (x, y, z)| {
            acc + x
                .intersection(&y)
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&z)
                .sum::<u32>()
        });

    println!("Day 3 Part 2: {res}");
}

fn main() {
    runner(part1);
    runner(part2);
}
