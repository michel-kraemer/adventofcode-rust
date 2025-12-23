# cargo bench-aoc

A command-line tool to benchmark my solutions to the Advent of Code. 

The tool uses [Criterion.rs](https://github.com/criterion-rs/criterion.rs) under the hood to provide accurate and reproducible benchmark results. I/O (i.e. reading input files and printing the answers to the terminal) is excluded.

Since all my solutions are self-contained programs, bench-aoc uses a little bit of trickery to temporarily convert a project to a benchmark:

* The tool copies the project to the temporary subdirectory `target/bench-aoc`.
* It then modifies the `Cargo.toml` in the temporary directory to add Criterion.rs as a dependency and to configure `cargo bench`.
* Further, it patches the temporary `main.rs` file as follows:
  * It adds boilerplate code to benchmark the main function with Criterion.rs.
  * It replaces `fs::read_to_string` statements with the contents of the puzzle's input file.
  * It replaces `println!` with `format!` so the answers will not be printed to the terminal anymore. Formatting still happens, though, as it might be part of the solution.
* Finally, the tool calls `cargo bench` from the temporary directory.

Note that the tool does not delete the temporary directory. This is useful as it allows you to modify the original program, run the benchmark, and see how the modifications have affected the performance. It also avoids unnecessary recompilation.

## Installation

To install the tool, run the following command in the `cargo-bench-aoc` directory:

```bash
cargo install --path .
```

## Usage

After installing, change into the project directory you want to benchmark and simply run

```bash
cargo bench-aoc .
```

You may also provide the path to the directory as the first parameter

```bash
cargo bench-aoc 2025/day01
```

or benchmark several projects (sequentially)

```bash
cargo bench-aoc 2025/day01 2025/day02 2025/day03
```
