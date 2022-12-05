use scan_fmt::scan_fmt;
use std::fs;

fn example_stacks() -> Vec<Vec<char>> {
    /*
        [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3
    */
    vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
}

fn input_stacks() -> Vec<Vec<char>> {
    /*
            [G]         [D]     [Q]
    [P]     [T]         [L] [M] [Z]
    [Z] [Z] [C]         [Z] [G] [W]
    [M] [B] [F]         [P] [C] [H] [N]
    [T] [S] [R]     [H] [W] [R] [L] [W]
    [R] [T] [Q] [Z] [R] [S] [Z] [F] [P]
    [C] [N] [H] [R] [N] [H] [D] [J] [Q]
    [N] [D] [M] [G] [Z] [F] [W] [S] [S]
    1   2   3   4   5   6   7   8   9
    */
    vec![
        vec!['N', 'C', 'R', 'T', 'M', 'Z', 'P'],
        vec!['D', 'N', 'T', 'S', 'B', 'Z'],
        vec!['M', 'H', 'Q', 'R', 'F', 'C', 'T', 'G'],
        vec!['G', 'R', 'Z'],
        vec!['Z', 'N', 'R', 'H'],
        vec!['F', 'H', 'S', 'W', 'P', 'Z', 'L', 'D'],
        vec!['W', 'D', 'Z', 'R', 'C', 'G', 'M'],
        vec!['S', 'J', 'F', 'L', 'H', 'W', 'Z', 'Q'],
        vec!['S', 'Q', 'P', 'W', 'N'],
    ]
}

fn read_moves(filename: &str) -> Vec<(u32, usize, usize)> {
    fs::read_to_string(filename)
        .expect("Failed to read input.txt")
        .lines()
        .map(|line| {
            let res = scan_fmt!(line, "move {d} from {d} to {d}", u32, usize, usize);
            let (a, b, c) = res.unwrap();
            (a, b - 1, c - 1)
        })
        .collect()
}

fn part1() {
    // let mut stacks = example_stacks();
    // let moves = read_moves("input_example.txt");
    let mut stacks = input_stacks();
    let moves = read_moves("input.txt");

    // println!("Stacks: {:?}", stacks);
    for (qty, src, dst) in moves {
        for _ in 0..qty {
            let cr = stacks[src].pop().unwrap();
            stacks[dst].push(cr);
        }
        // println!("Stacks: {:?}", stacks);
    }

    println!("Part 1: ");
    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!();
}

fn part2() {
    // let mut stacks = example_stacks();
    // let moves = read_moves("input_example.txt");
    let mut stacks = input_stacks();
    let moves = read_moves("input.txt");

    // println!("Stacks: {:?}", stacks);
    for (qty, src, dst) in moves {
        let l = stacks[src].len();
        let x: Vec<char> = stacks[src].drain(l - qty as usize..).collect();
        stacks[dst].extend(x);
        // println!("Stacks: {:?}", stacks);
    }

    println!("Part 2: ");
    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!();
}

fn main() {
    // part1();
    part2();
    println!("Hello, world!");
}
