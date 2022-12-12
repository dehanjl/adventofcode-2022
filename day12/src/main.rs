use hashbrown::HashMap;
use priority_queue::DoublePriorityQueue;
use std::{fs, i32::MAX};

type HeightMap = Vec<Vec<i32>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Loc {
    x: i32,
    y: i32,
}

impl Loc {
    fn find_neighbors(&self, heightmap: &HeightMap) -> Vec<Loc> {
        const DIRMAP: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        DIRMAP
            .iter()
            .map(|(dx, dy)| (self.x + dx, self.y + dy))
            .map(|(x, y)| {
                (
                    x,
                    y,
                    heightmap
                        .get(y as usize)
                        .and_then(|row| row.get(x as usize)),
                )
            })
            .filter(|(_, _, height)| height.is_some())
            .map(|(x, y, height)| (x, y, height.unwrap()))
            .filter(|(_, _, height)| heightmap[self.y as usize][self.x as usize] + 1 >= **height)
            .map(|(x, y, _)| Loc { x, y })
            .collect::<Vec<_>>()
    }
}

fn read_file(filename: &str) -> (HeightMap, Loc, Loc) {
    let (mut start, mut end) = (Loc { x: 0, y: 0 }, Loc { x: 0, y: 0 });
    let heightmap = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = Loc {
                            x: x as i32,
                            y: y as i32,
                        };
                        0
                    }
                    'E' => {
                        end = Loc {
                            x: x as i32,
                            y: y as i32,
                        };
                        25
                    }
                    _ => ('a'..='z').position(|t| t == c).unwrap() as i32,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    (heightmap, start, end)
}

fn a_star(heightmap: &HeightMap, start: Loc, end: Loc) -> i32 {
    /// Manhattan distance heuristic function.
    /// Because we can't step diagonally, this is admissable.
    fn h(loc: &Loc, end: &Loc) -> i32 {
        let dx = (loc.x - end.x).abs();
        let dy = (loc.y - end.y).abs();
        dx + dy
    }

    let mut open_set: DoublePriorityQueue<Loc, i32> = DoublePriorityQueue::new();
    let mut g_scores: HashMap<Loc, i32> = HashMap::new();
    g_scores.insert(start, 0);
    let mut f_scores: HashMap<Loc, i32> = HashMap::new();
    f_scores.insert(start, h(&start, &end));

    open_set.push(start, *f_scores.get(&start).unwrap());

    while !open_set.is_empty() {
        let current = open_set.pop_min().unwrap().0;
        if current == end {
            return *g_scores.get(&current).unwrap();
        }

        for neighbor in current.find_neighbors(&heightmap) {
            let tentative_g_score = g_scores[&current] + 1;
            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&MAX) {
                g_scores.insert(neighbor, tentative_g_score);
                f_scores.insert(neighbor, tentative_g_score + h(&neighbor, &end));
                open_set.push(neighbor, *f_scores.get(&neighbor).unwrap());
            }
        }
    }

    MAX
}

fn part1() {
    let (heightmap, start, end) = read_file("input.txt");
    println!("Part 1: {}", a_star(&heightmap, start, end));
}

fn part2() {
    let (heightmap, _, end) = read_file("input.txt");
    let mut starts: Vec<Loc> = Vec::new();

    for (y, row) in heightmap.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                starts.push(Loc {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    let min = starts
        .iter()
        .map(|&start| a_star(&heightmap, start, end))
        .min()
        .unwrap();

    println!("Part 2: {:?}", min);
}

fn main() {
    part1();
    part2();
}
