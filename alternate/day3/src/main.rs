use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input(filename: &str) -> Vec<Vec<u32>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| ('a'..='z').chain('A'..='Z').position(|x| x == c).unwrap() as u32 + 1)
                .collect()
        })
        .collect()
}

fn part1() {
    let res: u32 = read_input("input.txt")
        .iter()
        .map(|v| v.split_at(v.len() / 2))
        .map(|t| {
            (
                t.0.iter().cloned().collect::<HashSet<_>>(),
                t.1.iter().cloned().collect::<HashSet<_>>(),
            )
        })
        .fold(0, |acc, (x, y)| acc + x.intersection(&y).sum::<u32>());

    println!("Part 1: {}", res);
}

fn part2() {
    let res = read_input("input.txt")
        .chunks(3)
        .map(|v| {
            (
                v[0].iter().cloned().collect::<HashSet<_>>(),
                v[1].iter().cloned().collect::<HashSet<_>>(),
                v[2].iter().cloned().collect::<HashSet<_>>(),
            )
        })
        .fold(0, |acc, (x, y, z)| {
            acc + x
                .intersection(&y)
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&z)
                .sum::<u32>()
        });

    println!("Part 2: {}", res);
}

fn main() {
    part1();
    part2();
}
