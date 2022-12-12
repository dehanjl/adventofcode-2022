use itertools::Itertools;
use std::fs;

fn part1() {
    let chars = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    for i in 0..chars.len() {
        let slice = &chars[i..i + 4];
        if slice.into_iter().unique().count() == 4 {
            println!("Part 1: {}", i + 4);
            return;
        }
    }
}

fn part2() {
    let chars = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    for i in 0..chars.len() {
        let slice = &chars[i..i + 14];
        if slice.into_iter().unique().count() == 14 {
            println!("Part 1: {}", i + 14);
            return;
        }
    }
}

fn main() {
    part1();
    part2();
}
