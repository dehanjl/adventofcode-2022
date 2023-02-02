use std::env::{self, VarError};
use std::error::Error;
use std::path::{Path, PathBuf};

use reqwest::header::COOKIE;

fn main() {
    match download_input(2022, 1) {
        Ok(resp) => println!("{resp}"),
        Err(err) => panic!("Error: {err}"),
    }
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
