use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input(filename: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split(',')
                .map(|v| v.to_owned())
                .collect::<Vec<_>>()
        })
        .map(|s| {
            s.split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .map(|(r, f)| (r..=f).collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
        .collect()
}

fn part1() {
    let sum = read_input("input.txt").iter().fold(0, |acc, (x, y)| {
        let i = x.intersection(y).collect::<Vec<_>>();
        if i.len() == x.len() || i.len() == y.len() {
            return acc + 1;
        }
        acc
    });

    println!("Part 1: {}", sum);
}

fn part2() {
    let sum = read_input("input.txt").iter().fold(0, |acc, (x, y)| {
        let i = x.intersection(y).collect::<Vec<_>>();
        if i.len() > 0 {
            return acc + 1;
        }
        acc
    });

    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}
