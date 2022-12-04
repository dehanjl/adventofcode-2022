use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input(filename: &str) -> Vec<HashSet<u32>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|v| v.to_owned())
                .collect::<Vec<_>>()
        })
        .flat_map(|s| s)
        .map(|s| {
            s.split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .map(|(r, f)| (r..=f).collect::<HashSet<_>>())
        .collect::<Vec<_>>()
}

fn part1() {
    let sets = read_input("input.txt");
    let pairs = sets
        .chunks(2)
        .map(|s| {
            let mut s = s.iter();
            let a = s.next().cloned().unwrap();
            let b = s.next().cloned().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    pairs.iter().for_each(|(x, y)| {
        let i = x.intersection(y).collect::<Vec<_>>();
        // println!("{:?}", i);
        if i.len() == x.len() || i.len() == y.len() {
            sum += 1;
        }
    });

    println!("Part 1: {}", sum);
}

fn part2() {
    let sets = read_input("input.txt");
    let pairs = sets
        .chunks(2)
        .map(|s| {
            let mut s = s.iter();
            let a = s.next().cloned().unwrap();
            let b = s.next().cloned().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    pairs.iter().for_each(|(x, y)| {
        let i = x.intersection(y).collect::<Vec<_>>();
        // println!("{:?}", i);
        if i.len() > 0 {
            sum += 1;
        }
    });

    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}
