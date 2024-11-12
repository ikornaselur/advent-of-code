# Advent of Code

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
