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

fn part2() {
    let mut cal_vec = read_file("input.txt");
    let mut top_three: Vec<u32> = Vec::new();
    for _ in 0..3 {
        top_three.push(cal_vec.iter().max().unwrap().clone());
        cal_vec.retain(|&x| x != *top_three.last().unwrap());
    }

    println!("Part 2: {}, {:?}", top_three.iter().sum::<u32>(), top_three);
}

fn main() {
    part1();
    part2();
}
