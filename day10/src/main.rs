use std::fs;

fn read_input(filename: &str) -> Vec<i32> {
    let mut v = Vec::new();

    let mut reg_x = 1;
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .for_each(|line| {
            v.push(reg_x);

            match line.trim() {
                "noop" => {}
                _ => {
                    v.push(reg_x);

                    let val = line
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                    reg_x += val;
                }
            }
        });

    v
}

fn part1() {
    fn check(clk: usize) -> bool {
        clk == 20 || clk == 60 || clk == 100 || clk == 140 || clk == 180 || clk == 220
    }

    let mut sum = 0;

    read_input("input.txt")
        .iter()
        .enumerate()
        .for_each(|(clk, &reg_x)| {
            if check(clk + 1) {
                sum += (clk + 1) as i32 * reg_x;
            }
        });

    println!("Part 1: {}", sum);
}

fn part2() {
    let mut pixels: Vec<char> = Vec::new();
    pixels.resize(6 * 40, ' ');

    read_input("input.txt")
        .iter()
        .enumerate()
        .for_each(|(clk, &reg_x)| {
            let hor_pos = clk % 40;
            if hor_pos == (reg_x - 1) as usize
                || hor_pos == reg_x as usize
                || hor_pos == (reg_x + 1) as usize
            {
                pixels[clk] = '█';
            }
        });

    pixels.resize(6 * 40, '.');

    pixels
        .chunks(40)
        .for_each(|row| println!("{:?}", row.iter().collect::<String>()));
}

fn main() {
    part1();
    part2();
}
