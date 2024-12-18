# Advent of Code

The goal is to eventually solve all the AoC problems within this project. Below is the current progress.

It's gonna be a while before it's all golden stars!

<div align="center">
    <img src="https://aoc-stars.fly.dev/stars/ikornaselur/advent-of-code/main/.stars.svg" alt="Stars"/>
</div>

Each year is in its own year folder, work from within there

> [!NOTE]
> Use double digits, starting at 01

## Running

```bash
cargo test -p prob<int>  # run the tests for a problem
cargo run -p prob<int>  # run a problem, from prob01 to (hopefully) prob24
```

## Adding a new problem

1. Add "prob<int>" to members of the workspace
2. `cp -r .template prob<int>`
3. `cargo init prob<int>`

## Running with a file name argument

To run the app with a file name argument, use the following command:

```bash
cargo run -p prob<int> -- <file_name>
```

Replace `<file_name>` with the path to the file containing the input.
