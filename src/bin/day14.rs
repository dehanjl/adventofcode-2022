use adventofcode_2022::runner;
use itertools::Itertools;
use nalgebra::{DMatrix, Dynamic, Matrix, VecStorage};

type Path = Vec<(usize, usize)>;
type Cavern = Matrix<char, Dynamic, Dynamic, VecStorage<char, Dynamic, Dynamic>>;

fn draw_path(cavern: &mut Cavern, path: &Path, x_offset: usize) {
    for i in 1..path.len() {
        let (start, stop) = (path[i - 1], path[i]);
        for x in (start.0..=stop.0).chain(stop.0..=start.0) {
            cavern[(start.1, x - x_offset)] = '#';
        }
        for y in (start.1..=stop.1).chain(stop.1..=start.1) {
            cavern[(y, stop.0 - x_offset)] = '#';
        }
    }
}

fn parse_input(input: &str, add_floor: bool) -> (Cavern, usize) {
    let mut paths: Vec<Path> = input
        .lines()
        .map(|line| {
            line.split("->")
                .map(|s| s.trim().split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect()
        })
        .collect();

    let mut y_max = paths
        .iter()
        .flat_map(|it| it.iter().map(|(_, y)| *y))
        .max()
        .unwrap();

    if add_floor {
        y_max += 2;
        let floor_start = (500 - y_max, y_max);
        let floor_stop = (500 + y_max, y_max);
        paths.push(vec![floor_start, floor_stop]);
    }

    let x_max = paths
        .iter()
        .flat_map(|it| it.iter().map(|(x, _)| *x))
        .max()
        .unwrap();

    let x_min = paths
        .iter()
        .flat_map(|it| it.iter().map(|(x, _)| *x))
        .min()
        .unwrap();

    let mut cavern = DMatrix::from_element(y_max + 1, x_max - x_min + 1, '.');

    cavern[(0, 500 - x_min)] = '+';

    for path in paths {
        draw_path(&mut cavern, &path, x_min);
    }

    (cavern, x_min)
}

fn fall_sand(cavern: &mut Cavern, x_offset: usize) -> Result<(usize, usize), ()> {
    let mut pos: (usize, usize) = (500 - x_offset, 0);
    'outer: loop {
        for x in [(0, 1), (-1, 1), (1, 1)].iter() {
            let new_pos = ((pos.0 as i32 + x.0) as usize, pos.1 + x.1);
            match cavern.get((new_pos.1, new_pos.0)) {
                None => return Err(()), // we fall over the edge
                Some('.') => {
                    pos = new_pos;
                    continue 'outer; // there is an open spot
                }
                _ => {}
            }
        }

        if let Some('+') = cavern.get((pos.1, pos.0)) {
            return Err(()); // we hit the source
        }

        cavern[(pos.1, pos.0)] = 'O';
        return Ok(pos); // we found a place to rest
    }
}

fn part1(input: &str) {
    let (mut cavern, x_offset) = parse_input(input, false);

    let mut sand_count = 0;
    while fall_sand(&mut cavern, x_offset).is_ok() {
        sand_count += 1;
    }

    println!("{}", cavern);
    println!("Day 14 Part 1: {}", sand_count);
}

fn part2(input: &str) {
    let (mut cavern, x_offset) = parse_input(input, true);

    let mut sand_count = 0;
    while fall_sand(&mut cavern, x_offset).is_ok() {
        sand_count += 1;
    }

    println!("Day 14 Part 2: {}", sand_count + 1);
}

fn main() {
    runner(part1);
    runner(part2);
}
