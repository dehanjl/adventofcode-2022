use std::fs;

use hashbrown::HashSet;
#[macro_use]
extern crate scan_fmt;

type Instruction = (char, i32);
type Loc = (i32, i32);

trait LocUtils {
    fn is_adjacent(&self, other: &Loc) -> bool;
    fn move_self(&mut self, dir: char);
    fn follow(&mut self, other: &Loc);
}

impl LocUtils for Loc {
    fn is_adjacent(&self, other: &Loc) -> bool {
        (self.0 - other.0).abs() <= 1 && (self.1 - other.1).abs() <= 1
    }

    fn move_self(&mut self, dir: char) {
        match dir {
            'U' => self.1 += 1,
            'D' => self.1 -= 1,
            'R' => self.0 += 1,
            'L' => self.0 -= 1,
            _ => panic!("Invalid direction"),
        }
    }

    fn follow(&mut self, other: &Loc) {
        self.0 += (other.0 - self.0).signum();
        self.1 += (other.1 - self.1).signum();
    }
}

fn read_file(filename: &str) -> Vec<Instruction> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| scan_fmt!(line, "{} {}", char, i32).unwrap())
        .collect::<Vec<Instruction>>()
}

fn simulate(instructions: &Vec<Instruction>, size: usize) -> HashSet<Loc> {
    let mut rope: Vec<Loc> = vec![(0, 0); size];
    let mut visited: HashSet<Loc> = HashSet::new();

    for (dir, amt) in instructions {
        for _ in 0..*amt {
            rope[0].move_self(*dir);

            for i in 1..rope.len() {
                let (follower, leader) = (rope[i], rope[i - 1]);
                if !leader.is_adjacent(&follower) {
                    rope[i].follow(&leader);
                }
            }

            visited.insert(rope[rope.len() - 1]);
        }
    }

    visited
}

fn part1() {
    println!("Part 1: {}", simulate(&read_file("input.txt"), 2).len());
}

fn part2() {
    println!("Part 2: {}", simulate(&read_file("input.txt"), 10).len());
}

fn main() {
    part1();
    part2();
}
