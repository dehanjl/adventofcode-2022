extern crate nalgebra as na;
use std::fs;

use na::{DMatrix, Dynamic, Matrix, VecStorage};

type Forest = Matrix<u16, Dynamic, Dynamic, VecStorage<u16, Dynamic, Dynamic>>;

fn read_file(filename: &str) -> Forest {
    let input = fs::read_to_string(filename).unwrap();
    let y = input.lines().count();
    let x = input.lines().next().unwrap().len();

    let mut val_mat: Forest = DMatrix::from_element(y, x, 0 as u16);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            val_mat[(y, x)] = c.to_digit(10).unwrap() as u16;
        });
    });

    val_mat
}

fn split_skip_mid(v: &Vec<u16>, mid: usize) -> (Vec<u16>, Vec<u16>) {
    let (a, b) = v.split_at(mid);
    (a[..].to_vec(), b[1..].to_vec())
}

fn part1() {
    let forest = read_file("input.txt");
    let mut vis = DMatrix::from_element(forest.nrows(), forest.ncols(), false);

    // set all trees on the outside as visible
    for y in 0..forest.nrows() {
        for x in 0..forest.ncols() {
            if x == 0 || y == 0 || x == forest.ncols() - 1 || y == forest.nrows() - 1 {
                vis[(y, x)] = true;
            }
        }
    }

    // check the rows for visible trees
    for (y, row) in forest.row_iter().enumerate() {
        for (x, &val) in row.into_iter().enumerate() {
            let r = row.iter().cloned().collect::<Vec<_>>();

            let (a, b) = split_skip_mid(&r, x);
            let (&a, &b) = (a.iter().max().unwrap_or(&0), b.iter().max().unwrap_or(&0));

            if val > a || val > b {
                vis[(y, x)] = true;
            }
        }
    }

    // check the cols for visible trees
    for (x, col) in forest.column_iter().enumerate() {
        for (y, &val) in col.into_iter().enumerate() {
            let r = col.iter().cloned().collect::<Vec<_>>();

            let (a, b) = split_skip_mid(&r, y);
            let (&a, &b) = (a.iter().max().unwrap_or(&0), b.iter().max().unwrap_or(&0));

            if val > a || val > b {
                vis[(y, x)] = true;
            }
        }
    }

    let res = vis.iter().filter(|&&v| v).count();
    println!("Part 1: {}", res);
}

fn part2() {
    let forest = read_file("input.txt");

    let mut scores: Vec<u32> = Vec::new();

    forest.row_iter().enumerate().for_each(|(y, col)| {
        col.iter().enumerate().for_each(|(x, &val)| {
            let r = forest
                .row_iter()
                .nth(y)
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<_>>();

            let c = forest
                .column_iter()
                .nth(x)
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<_>>();

            let (mut left, right) = split_skip_mid(&r, x);
            let (mut up, down) = split_skip_mid(&c, y);
            left.reverse();
            up.reverse();

            let lscore = direction_score(&left, val);
            let rscore = direction_score(&right, val);
            let uscore = direction_score(&up, val);
            let dscore = direction_score(&down, val);

            let score = lscore * rscore * uscore * dscore;

            scores.push(score);
        });
    });

    println!("Part 2: {}", scores.iter().max().unwrap());
}

fn direction_score(v: &Vec<u16>, val: u16) -> u32 {
    let mut score = v.iter().take_while(|&&h| h < val).count();
    match v.iter().nth(score) {
        Some(&h) if h >= val => score += 1,
        _ => (),
    }
    score as u32
}

fn main() {
    part1();
    part2();
}
