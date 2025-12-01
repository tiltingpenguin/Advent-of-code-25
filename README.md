# My Solutions for Advent of Code 2025

## Setup

To generate the boilerplate and test setup for each day run:

`cargo generate -p ./template -n dayXX`

Common dependencies for parsing (nom) and alogorithms are included.

Copy the test input from the Advent of Code page for each day into `dayXX-test.txt` and the actual input into `dayXX-input.txt`.
Then run the tests with

`cargo test dayXX-01` and `cargo test dayXX-02`

to get the real solution with your input run

`cargo run --bin dayXX-01` and `cargo run --bin dayXX-02`
