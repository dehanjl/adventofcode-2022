use std::collections::VecDeque;

use adventofcode_2022::runner;
use indicatif::ProgressBar;

const WIDTH: usize = 7;
const SHAPE_ORDER: [ShapeType; 5] = [
    ShapeType::Horizontal,
    ShapeType::Cross,
    ShapeType::Hook,
    ShapeType::Vertical,
    ShapeType::Box,
];

type Board = VecDeque<Vec<bool>>;

enum MoveError {
    Collision,
    Settle,
}

#[derive(Copy, Clone)]
enum MoveDirection {
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum ShapeType {
    Horizontal,
    Cross,
    Hook,
    Vertical,
    Box,
}

#[derive(Clone)]
struct Shape {
    shape_type: ShapeType,
    positions: Vec<(usize, usize)>,
}

impl Shape {
    fn new(shape_type: ShapeType) -> Self {
        match shape_type {
            ShapeType::Horizontal => Self {
                shape_type,
                positions: vec![(2, 0), (3, 0), (4, 0), (5, 0)],
            },
            ShapeType::Cross => Self {
                shape_type,
                positions: vec![(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)],
            },
            ShapeType::Hook => Self {
                shape_type,
                positions: vec![(4, 0), (4, 1), (2, 2), (3, 2), (4, 2)],
            },
            ShapeType::Vertical => Self {
                shape_type,
                positions: vec![(2, 0), (2, 1), (2, 2), (2, 3)],
            },
            ShapeType::Box => Self {
                shape_type,
                positions: vec![(2, 0), (3, 0), (2, 1), (3, 1)],
            },
        }
    }

    fn lines_required(&self) -> i32 {
        match self.shape_type {
            ShapeType::Horizontal => 4,
            ShapeType::Cross => 6,
            ShapeType::Hook => 6,
            ShapeType::Vertical => 7,
            ShapeType::Box => 5,
        }
    }

    fn relocate(&self, direction: MoveDirection, board: &Board) -> Result<Shape, MoveError> {
        self.positions
            .iter()
            .map(|(x, y)| match direction {
                MoveDirection::Down => {
                    if y + 1 >= board.len() || board[y + 1][*x] {
                        return Err(MoveError::Settle);
                    }
                    Ok((*x, y + 1))
                }
                MoveDirection::Left => {
                    if x.eq(&0) || board[*y].get(x - 1).is_none() || board[*y][x - 1] {
                        return Err(MoveError::Collision);
                    }
                    Ok((x - 1, *y))
                }
                MoveDirection::Right => {
                    if board[*y].get(x + 1).is_none() || board[*y][x + 1] {
                        return Err(MoveError::Collision);
                    }
                    Ok((x + 1, *y))
                }
            })
            .collect::<Result<Vec<_>, MoveError>>()
            .and_then(|new_pos| {
                Ok(Shape {
                    shape_type: self.shape_type,
                    positions: new_pos,
                })
            })
    }
}

trait BoardUtils {
    fn inject_space(&mut self, amt: i32);
    fn trim_space(&mut self);
}

impl BoardUtils for Board {
    fn inject_space(&mut self, amt: i32) {
        let free = self
            .iter()
            .take_while(|row| row.iter().all(|cell| !cell))
            .count() as i32;
        for _ in 0..((amt - free).max(0)) {
            self.push_front(vec![false; WIDTH]);
        }
    }

    fn trim_space(&mut self) {
        let free = self
            .iter()
            .take_while(|row| row.iter().all(|cell| !cell))
            .count() as i32;
        for _ in 0..free {
            self.pop_front();
        }
    }
}

fn parse_input(input: &str) -> Vec<MoveDirection> {
    input
        .chars()
        .map(|c| match c {
            '<' => MoveDirection::Left,
            '>' => MoveDirection::Right,
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn display_board(board: &Board, shape: &Shape) {
    for (y, row) in board.iter().enumerate() {
        print!("|");
        for (x, cell) in row.iter().enumerate() {
            print!(
                "{}",
                if *cell {
                    '#'
                } else if shape.positions.contains(&(x, y)) {
                    '@'
                } else {
                    '.'
                }
            );
        }
        println!("|");
    }
    println!("+-------+\n");
}

fn part1(input: &str) {
    let jet_stream = parse_input(input);
    let mut board: Board = vec![vec![false; WIDTH]; 0].into();

    let mut move_count = 0;
    for i in 0..2022 {
        let mut shape = Shape::new(SHAPE_ORDER[i % 5]);
        board.trim_space();
        board.inject_space(shape.lines_required());

        while {
            let move_dir;
            if move_count % 2 == 0 {
                move_dir = jet_stream[(move_count / 2) % jet_stream.len()];
            } else {
                move_dir = MoveDirection::Down;
            }
            move_count += 1;
            let res = shape.relocate(move_dir, &board);
            match res {
                Ok(new_shape) => {
                    shape = new_shape;
                    true
                }
                Err(MoveError::Collision) => true,
                Err(MoveError::Settle) => {
                    for (x, y) in shape.positions.iter() {
                        board[*y][*x] = true;
                    }
                    false
                }
            }
        } {}
    }
    board.trim_space();
    println!("Day 17 Part 1: {}", board.len());
}

fn part2(input: &str) {}

fn main() {
    runner(part1);
}
