use std::fs;

pub fn runner(f: impl Fn(&str), filepath: &str) {
    // TODO: let this determine the filepath at runtime
    let input = fs::read_to_string(filepath).expect("Oh no! I couldn't read the file :(");
    f(&input);
}

// TODO: add the structopt
