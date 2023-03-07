use adventofcode_2022::runner;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

type Stack = HashMap<usize, Vec<char>>;
type Moves = Vec<(u32, usize, usize)>;
fn parse_input(input: &str) -> (Stack, Moves) {
    let moves = input
        .lines()
        .filter(|&l| l.starts_with("move"))
        .map(|line| {
            let (qty, src, dst) =
                scan_fmt!(line, "move {d} from {d} to {d}", u32, usize, usize).unwrap();
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

fn part1(input: &str) {
    let (mut stacks, moves) = parse_input(input);

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

fn part2(input: &str) {
    let (mut stacks, moves) = parse_input(input);

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
    runner(part1);
    runner(part2);
}
