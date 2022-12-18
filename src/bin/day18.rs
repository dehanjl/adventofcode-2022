use adventofcode_2022::runner;
use itertools::Itertools;
use scan_fmt::scan_fmt;

type Loc = (usize, usize, usize);

fn parse_input(input: &str) -> Vec<Loc> {
    input
        .lines()
        .map(|line| scan_fmt!(line, "{},{},{}", usize, usize, usize))
        .flatten()
        .collect()
}

fn part1(input: &str) {
    let coords = parse_input(input);
    let max = *coords.iter().flat_map(|(x, y, z)| [x, y, z]).max().unwrap();

    let mut grid = vec![vec![vec![false; max + 2]; max + 2]; max + 2];
    for (x, y, z) in coords {
        grid[x][y][z] = true;
    }

    let mut surface_area = 0;
    for i in 1..=max {
        for j in 1..=max {
            for k in 1..=max {
                if grid[i][j][k] {
                    let sides = [
                        (i - 1, j, k),
                        (i + 1, j, k),
                        (i, j - 1, k),
                        (i, j + 1, k),
                        (i, j, k - 1),
                        (i, j, k + 1),
                    ];
                    for (a, b, c) in sides {
                        if !grid[a][b][c] {
                            surface_area += 1;
                        }
                    }
                }
            }
        }
    }

    println!("Day 18 Part 1: {}", surface_area);
}

fn part2(input: &str) {}

fn main() {
    runner(part1);
    runner(part1x);
}

use hashbrown::HashSet;

fn sides((x, y, z): (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn part1x(input: &str) {
    let drops = input
        .lines()
        .filter_map(|l| l.split(',').map(|x| x.parse().unwrap()).collect_tuple())
        .collect::<HashSet<_>>();
    let p1 = drops
        .iter()
        .flat_map(|&p| sides(p))
        .filter(|s| !drops.contains(s))
        .count();

    println!("Day 18 Part 1: {}", p1);
}
