use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::{collections::HashMap, fs};

fn read_input(filename: &str) -> (HashMap<usize, Vec<char>>, Vec<(u32, usize, usize)>) {
    let input = fs::read_to_string(filename).unwrap();

    let moves = input
        .lines()
        .filter(|&l| l.starts_with("move"))
        .map(|line| {
            let (qty, src, dst) =
                scan_fmt!(&line, "move {d} from {d} to {d}", u32, usize, usize).unwrap();
            (qty, src - 1, dst - 1)
        })
        .collect();

    let mut stacks = input
        .lines()
        .take_while(|&l| !l.starts_with("move"))
        .flat_map(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|&(_, c)| c.is_alphabetic())
        })
        .into_group_map();

    stacks.iter_mut().for_each(|(_, v)| v.reverse());

    (stacks, moves)
}

fn part1(filename: &str) {
    let (mut stacks, moves) = read_input(filename);

    moves.iter().for_each(|(qty, src, dst)| {
        (0..*qty).for_each(|_| {
            let cat = stacks.get_mut(src).unwrap().pop().unwrap();
            stacks.get_mut(dst).unwrap().push(cat)
        });
    });

    print!("Part 1: ");
    for i in 0..stacks.len() {
        print!("{}", stacks[&i].last().unwrap());
    }
    println!()
}

fn part2(filename: &str) {
    let (mut stacks, moves) = read_input(filename);

    moves.iter().for_each(|(qty, src, dst)| {
        let l = stacks[src].len();
        let x: Vec<char> = stacks
            .get_mut(src)
            .unwrap()
            .drain(l - *qty as usize..)
            .collect();
        stacks.get_mut(dst).unwrap().extend(x);
    });

    print!("Part 2: ");
    for i in 0..stacks.len() {
        print!("{}", stacks[&i].last().unwrap());
    }
    println!()
}

fn main() {
    let filename = "input.txt";
    part1(filename);
    part2(filename);
}
