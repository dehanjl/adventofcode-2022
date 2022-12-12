# Advent of Code 2022
A (mostly) Rust solution to the [Advent of Code](https://adventofcode.com/) puzzles for 2022.

## Instructions
Run a day using `cargo run --bin <day>` where `<day>` is the day to run. 

## Folder Structure
```
.
├── alternate
│   └── dayX # Alternate/non-refactored/different language solutions to day X
├── inputs
│   ├── example # example puzzle inputs
│   │   └── dayX.txt
│   └── real # real puzzle inputs
│       └── dayX.txt
└── src
    ├── bin
    │   └── dayX.rs # solution for day X
    ├── lib.rs # helper library
    └── main.rs # main project binary, does nothing right now
```