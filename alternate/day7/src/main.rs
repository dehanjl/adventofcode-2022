use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug)]
enum Node {
    File((i64, String)),
    Directory((Option<i64>, String)),
}

type FileSystem = HashMap<PathBuf, Vec<Node>>;

fn calculate_sizes(filesys: &FileSystem, sizes: &mut HashMap<PathBuf, i64>, dir: &PathBuf) {
    if sizes.contains_key(dir) {
        return; // TODO: find a way of setting the values for directories once
    }

    let size: i64 = filesys[dir]
        .iter()
        .map(|node| match node {
            Node::File((size, _)) => *size,
            Node::Directory((None, name)) => {
                calculate_sizes(filesys, sizes, &dir.join(name.clone()));
                sizes[&dir.join(name.clone())]
            }
            Node::Directory((Some(size), _)) => *size,
        })
        .sum();

    sizes.insert(dir.clone(), size);
}

fn read_file(filename: &str) -> FileSystem {
    let mut filesys: FileSystem = HashMap::new();
    let mut path = PathBuf::new();

    fs::read_to_string(filename)
        .expect("Oh no! Could not read the file.")
        .split("$")
        .skip(1)
        .for_each(|command| match command.trim().lines().next().unwrap() {
            "ls" => {
                let nodes = command.lines().skip(1).map(|out| {
                    let (info, name) = out.split_once(' ').unwrap();
                    match info {
                        "dir" => Node::Directory((None, String::from(name))),
                        filesize => {
                            Node::File((filesize.parse::<i64>().unwrap(), String::from(name)))
                        }
                    }
                });
                filesys
                    .entry(path.clone())
                    .or_insert(Vec::new())
                    .extend(nodes);
            }
            "cd .." => {
                path.pop();
            }
            cd_dir => {
                path.push(cd_dir.split_whitespace().last().unwrap());
            }
        });

    filesys
}

fn part1() {
    let filesys: FileSystem = read_file("input.txt");
    let mut sizes: HashMap<PathBuf, i64> = HashMap::new();
    calculate_sizes(&filesys, &mut sizes, &PathBuf::from("/"));

    let res = sizes
        .iter()
        .map(&|(_, s)| s)
        .filter(|&&s| s <= 100_000)
        .sum::<i64>();

    println!("Part 1: {}", res)
}

fn part2() {
    let filesys: FileSystem = read_file("input.txt");
    let mut sizes: HashMap<PathBuf, i64> = HashMap::new();
    calculate_sizes(&filesys, &mut sizes, &PathBuf::from("/"));

    let total = sizes[&PathBuf::from("/")];

    let res = sizes
        .iter()
        .map(&|(_, &s)| s)
        .filter(|&s| total - s <= 40_000_000)
        .min()
        .unwrap()
        .clone();

    println!("Part 2: {:?}", res);
}

fn main() {
    part1();
    part2();
}
