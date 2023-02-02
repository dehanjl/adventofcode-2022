use std::env::VarError;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{env, fs};
use structopt::StructOpt;

use reqwest::header::COOKIE;

fn get_input(opt: &Opt) -> String {
    // find the name of the binary
    let bin = env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from);

    // and the day number from it
    let day_num = bin
        .and_then(|b| b.strip_prefix("day").map(String::from))
        .and_then(|b| b.parse::<u8>().ok());

    // try and read cached or example input
    let filepath = find_file(&opt);

    let x = match (filepath, opt.real) {
        (Some(filepath), _) => fs::read_to_string(filepath),
        (None, false) => panic!("Oh no! I couldn't find the example input file :("),
        (None, true) => {
            match day_num
                .map(|day_num| download_input(2022, day_num))
                .expect("Oh no! I couldn't download the input :(")
            {
                Ok(resp) => Ok(resp),
                Err(err) => err,
            }
        }
    };

    String::from("not there yet")
}

fn find_file(opt: &Opt) -> Option<PathBuf> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push("inputs");
    path.push(if opt.real { "real" } else { "example" });

    let bin = env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from);

    if let Some(bin) = bin {
        path.push(bin);
        path.set_extension("txt");
        if path.exists() {
            return Some(path);
        }
    }

    None
}

fn make_url(year: u16, day: u8) -> String {
    format!("https://adventofcode.com/{year}/day/{day}/input")
}

fn get_session_token() -> Result<String, VarError> {
    env::var("AOC_SESSION")
}

fn download_input(year: u16, day: u8) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    client
        .get(make_url(year, day))
        .header(
            COOKIE,
            "session=".to_owned() + get_session_token()?.as_str(),
        )
        .send()?
        .text()
        .map_err(|err| Box::new(err) as Box<dyn Error>)
}

pub fn runner(f: impl Fn(&str)) {
    let opt = Opt::from_args();

    println!("---");
    let start = Instant::now();
    f(&input);
    let duration = start.elapsed();
    println!("--- {:?}", duration)
}
#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    real: bool,
}

pub fn is_real() -> bool {
    let opt = Opt::from_args();
    opt.real
}
