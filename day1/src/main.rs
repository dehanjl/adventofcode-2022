use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file(filename: &str) -> Vec<u32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut cal: u32 = 0;
    let mut cal_vec: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() > 0 {
            cal += line.parse::<u32>().unwrap();
        } else {
            cal_vec.push(cal);
            cal = 0;
        }
    }

    cal_vec
}

fn part1() {
    println!("Part 1: {}", read_file("input.txt").iter().max().unwrap());
}

fn main() {
    part1();
}
