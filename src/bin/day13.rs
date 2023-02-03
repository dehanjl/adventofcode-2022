use adventofcode_2022::runner;
use itertools::Itertools;
use serde_json::Value;
use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<(Value, Value)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap()))
        .collect()
}

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a.as_i64().cmp(&b.as_i64()),
        (Value::Array(a), Value::Array(b)) => {
            let (mut a, mut b) = (a.iter(), b.iter());
            loop {
                // this can be replaced by a for loop, but this is more fun
                match (a.next(), b.next()) {
                    (None, None) => return Ordering::Equal, // two lists of equal length and values
                    (None, _) => return Ordering::Less,
                    (_, None) => return Ordering::Greater,
                    (Some(a), Some(b)) => match compare(a, b) {
                        Ordering::Equal => {}
                        ord => return ord,
                    },
                }
            }
        }
        (Value::Array(_), Value::Number(_)) => compare(a, &Value::Array(vec![b.clone()])),
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![a.clone()]), b),
        _ => unreachable!(),
    }
}

fn part1(input: &str) {
    let res = parse_input(input)
        .iter()
        .positions(|(left, right)| compare(left, right).is_lt())
        .map(|i| i + 1)
        .sum::<usize>();

    println!("Day 13 Part 1: {res:?}");
}

fn part2(input: &str) {
    let markers = [
        serde_json::from_str::<Value>("[[2]]").unwrap(),
        serde_json::from_str::<Value>("[[6]]").unwrap(),
    ];
    let res = parse_input(input)
        .iter()
        .flat_map(|(a, b)| [a, b])
        .chain(&markers)
        .cloned()
        .sorted_by(compare)
        .positions(|packet| markers.contains(&packet))
        .map(|i| i + 1)
        .product::<usize>();

    println!("Day 13 Part 2: {res:?}");
}

fn main() {
    runner(part1);
    runner(part2);
}
