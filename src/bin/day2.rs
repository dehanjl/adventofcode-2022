use adventofcode_2022::runner;

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for RPS {
    fn from(c: char) -> Self {
        let x = (c as i32) - ('A' as i32);
        match x {
            0 | 23 => RPS::Rock,
            1 | 24 => RPS::Paper,
            2 | 25 => RPS::Scissors,
            _ => panic!("Unknown opponent move"),
        }
    }
}

/// Does player beat opponent?
/// Then give scores as: win -> 6, draw -> 3, loss -> 0
fn beat(opponent: RPS, player: RPS) -> u32 {
    match (player, opponent) {
        (_, _) if opponent == player => 3,
        (RPS::Rock, RPS::Scissors) => 6,
        (RPS::Paper, RPS::Rock) => 6,
        (RPS::Scissors, RPS::Paper) => 6,
        _ => 0,
    }
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.split_whitespace().map(|x| x.chars().next().unwrap());
            (chars.next().unwrap(), chars.last().unwrap())
        })
        .collect()
}

fn part1(input: &str) {
    let score = parse_input(input)
        .iter()
        .map(|&(opponent, player)| (opponent.into(), player.into()))
        .fold(0, |acc, (opponent, player)| {
            acc + match player {
                RPS::Rock => 1,
                RPS::Paper => 2,
                RPS::Scissors => 3,
            } + beat(opponent, player)
        });

    println!("Day 2 Part 1: {score}");
}

fn part2(input: &str) {
    let score = parse_input(input)
        .iter()
        .map(|&(opponent, result)| (opponent.into(), result))
        .map(|(opponent, result)| {
            match (result, opponent) {
                // X -> loss
                ('X', RPS::Rock) => (opponent, RPS::Scissors),
                ('X', RPS::Paper) => (opponent, RPS::Rock),
                ('X', RPS::Scissors) => (opponent, RPS::Paper),
                // Y -> draw
                ('Y', _) => (opponent, opponent),
                // Z -> win
                ('Z', RPS::Rock) => (opponent, RPS::Paper),
                ('Z', RPS::Paper) => (opponent, RPS::Scissors),
                ('Z', RPS::Scissors) => (opponent, RPS::Rock),
                _ => panic!("Unknown result"),
            }
        })
        .fold(0, |acc, (opponent, player)| {
            acc + match player {
                RPS::Rock => 1,
                RPS::Paper => 2,
                RPS::Scissors => 3,
            } + beat(opponent, player)
        });

    println!("Day 2 Part 2: {score}");
}

fn main() {
    runner(part1);
    runner(part2);
}
