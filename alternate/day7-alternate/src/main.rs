use std::{fs, ops::AddAssign, path::PathBuf};

use hashbrown::{HashMap, HashSet};

fn read_file(filename: &str) -> HashMap<PathBuf, i64> {
    let mut dir_sizes: HashMap<PathBuf, i64> = HashMap::new();
    let mut seen_files: HashSet<PathBuf> = HashSet::new();
    let mut path = PathBuf::new();

    fs::read_to_string(filename)
        .expect("Oh no! Could not read the file.")
        .split('$')
        .skip(1)
        .for_each(|command| match command.trim().lines().next().unwrap() {
            "ls" => {
                command.lines().skip(1).for_each(|out| {
                    let (info, name) = out.split_once(' ').unwrap();
                    match info {
                        "dir" => {}
                        filesize => {
                            if !seen_files.contains(&path.join(name)) {
                                seen_files.insert(path.join(name));

                                let mut alt_path = path.clone();

                                while {
                                    // do while
                                    dir_sizes
                                        .entry(alt_path.clone())
                                        .or_insert(0)
                                        .add_assign(filesize.parse::<i64>().unwrap());
                                    alt_path.pop()
                                } {}
                            }
                        }
                    }
                });
            }
            "cd .." => {
                path.pop();
            }
            cd_dir => {
                path.push(cd_dir.split_whitespace().last().unwrap());
            }
        });

    dir_sizes
}

fn part1() {
    let res = read_file("input.txt")
        .values()
        .filter(|&&x| x <= 100_000)
        .sum::<i64>();
    println!("Part 1: {}", res);
}

fn part2() {
    let sizes = read_file("input.txt");
    let total = *sizes.values().max().unwrap();
    let res = *sizes
        .values()
        .filter(|&&x| total - x <= 40_000_000)
        .min()
        .unwrap();
    println!("Part 2: {}", res);
}

fn main() {
    part1();
    part2()
}
