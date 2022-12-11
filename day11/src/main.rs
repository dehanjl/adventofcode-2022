use std::fs;

struct Monkey {
    items: Vec<i64>,
    opp: Box<dyn Fn(i64) -> i64>,
    test: i64,
    targets: (usize, usize),
}

fn read_file(filename: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    fs::read_to_string(filename)
        .unwrap()
        .split("Monkey")
        .skip(1)
        .for_each(|monkey| {
            let mut lines = monkey.lines().skip(1).map(|l| l.trim());

            // find the starting items for a monkey
            let items = lines
                .next()
                .unwrap()
                .trim_start_matches("Starting items: ")
                .split(", ")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            // add the operation function to the monkey
            let opp: Box<dyn Fn(i64) -> i64> = match {
                let (opp, val) = lines
                    .next()
                    .unwrap()
                    .trim_start_matches("Operation: new = old ")
                    .split_once(' ')
                    .unwrap();

                (opp, val.parse::<i64>().ok())
            } {
                ("+", None) => Box::new(|old: i64| old + old),
                ("*", None) => Box::new(|old: i64| old * old),
                ("+", Some(value)) => Box::new(move |old: i64| old + value),
                ("*", Some(value)) => Box::new(move |old: i64| old * value),
                _ => panic!("Unknown opperand"),
            };

            // find the test value and test result behaviour for a monkey
            let test = lines
                .next()
                .unwrap()
                .trim_start_matches("Test: divisible by ")
                .parse::<i64>()
                .unwrap();

            let true_case = lines
                .next()
                .unwrap()
                .trim_start_matches("If true: throw to monkey ")
                .parse::<usize>()
                .unwrap();

            let false_case = lines
                .next()
                .unwrap()
                .trim_start_matches("If false: throw to monkey ")
                .parse::<usize>()
                .unwrap();

            let targets = (true_case, false_case);

            monkeys.push(Monkey {
                items,
                opp,
                test,
                targets,
            });
        });

    monkeys
}

fn play(monkeys: &mut Vec<Monkey>, max_rounds: usize, rule: impl Fn(i64) -> i64) -> Vec<usize> {
    let mut inspect_counts = vec![0; monkeys.len()];

    for _ in 0..max_rounds {
        for m in 0..monkeys.len() {
            for &item in monkeys[m].items.clone().iter() {
                inspect_counts[m] += 1;

                let new_item = (rule)((monkeys[m].opp)(item));

                let targets = monkeys[m].targets;
                if new_item % monkeys[m].test == 0 {
                    monkeys[targets.0].items.push(new_item);
                } else {
                    monkeys[targets.1].items.push(new_item);
                }
            }
            monkeys[m].items.clear();
        }
    }

    inspect_counts.sort();
    inspect_counts.reverse();
    inspect_counts
}

fn part1() {
    let mut monkeys = read_file("input.txt");
    let inspect_counts = play(&mut monkeys, 20, |i| i / 3);
    println!("Part 1: {:?}", inspect_counts[0] * inspect_counts[1]);
}

fn part2() {
    let mut monkeys = read_file("input.txt");
    let modulo = monkeys.iter().fold(1, |acc, m| acc * m.test);
    let inspect_counts = play(&mut monkeys, 10_000, |i| i % modulo);
    println!("Part 2: {:?}", inspect_counts[0] * inspect_counts[1]);
}

fn main() {
    part1();
    part2();
}
