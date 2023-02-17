use adventofcode_2022::runner;
use hashbrown::HashSet;
use scan_fmt::scan_fmt;

fn parse_input(input: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    input
        .lines()
        .flat_map(|line| scan_fmt!(line, "{d}-{d},{d}-{d}", u32, u32, u32, u32))
        .map(|(r1, f1, r2, f2)| {
            (
                (r1..=f1).collect::<HashSet<_>>(),
                (r2..=f2).collect::<HashSet<_>>(),
            )
        })
        .collect()
}

fn part1(input: &str) {
    let sum = parse_input(input)
        .iter()
        .filter(|(x, y)| {
            let i = x.intersection(y).collect::<Vec<_>>();
            i.len() == x.len() || i.len() == y.len()
        })
        .count();

    println!("Day 4 Part 1: {sum}");
}

fn part2(input: &str) {
    let sum = parse_input(input)
        .iter()
        .filter(|(x, y)| !x.intersection(y).collect::<Vec<_>>().is_empty())
        .count();

    println!("Day 4 Part 2: {sum}");
}

fn main() {
    runner(part1);
    runner(part2);
}
