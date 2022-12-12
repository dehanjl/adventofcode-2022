use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

/// Does player beat opponent?
/// Then give scores as: win -> 6, draw -> 3, loss -> 0
fn beat(opponent: RPS, player: RPS) -> u32 {
    if opponent == player {
        return 3; // draw
    }
    match (player, opponent) {
        (RPS::Rock, RPS::Scissors) => 6,
        (RPS::Paper, RPS::Rock) => 6,
        (RPS::Scissors, RPS::Paper) => 6,
        _ => 0,
    }
}

fn read_file(filename: &str) -> Vec<(char, char)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut chars = line.split_whitespace().map(|x| x.chars().next().unwrap());
            (chars.next().unwrap(), chars.last().unwrap())
        })
        .collect()
}

fn part1() {
    let moves = read_file("input.txt");

    let mut score = 0;
    moves
        .iter()
        .map(|&(opponent, player)| {
            (
                match opponent {
                    'A' => RPS::Rock,
                    'B' => RPS::Paper,
                    'C' => RPS::Scissors,
                    _ => panic!("Unknown opponent move"),
                },
                match player {
                    'X' => RPS::Rock,
                    'Y' => RPS::Paper,
                    'Z' => RPS::Scissors,
                    _ => panic!("Unknown player move"),
                },
            )
        })
        .for_each(|(opponent, player)| {
            match player {
                RPS::Rock => score += 1,
                RPS::Paper => score += 2,
                RPS::Scissors => score += 3,
            }
            score += beat(opponent, player);
        });

    println!("Part 1: {}", score);
}

fn part2() {
    let moves = read_file("input.txt");

    let mut score = 0;
    moves
        .iter()
        .map(|&(opponent, result)| {
            (
                match opponent {
                    'A' => RPS::Rock,
                    'B' => RPS::Paper,
                    'C' => RPS::Scissors,
                    _ => panic!("Unknown opponent move"),
                },
                result,
            )
        })
        .map(|(opponent, result)| {
            if result == 'Y' {
                return (opponent, opponent.clone()); // draw
            }
            match (result, opponent) {
                // X -> loss
                ('X', RPS::Rock) => (opponent, RPS::Scissors),
                ('X', RPS::Paper) => (opponent, RPS::Rock),
                ('X', RPS::Scissors) => (opponent, RPS::Paper),
                // Z -> win
                ('Z', RPS::Rock) => (opponent, RPS::Paper),
                ('Z', RPS::Paper) => (opponent, RPS::Scissors),
                ('Z', RPS::Scissors) => (opponent, RPS::Rock),
                _ => panic!("Unknown result"),
            }
        })
        .for_each(|(opponent, player)| {
            match player {
                RPS::Rock => score += 1,
                RPS::Paper => score += 2,
                RPS::Scissors => score += 3,
            }
            score += beat(opponent, player);
        });

    println!("Part 2: {}", score);
}

fn main() {
    part1();
    part2();
}
