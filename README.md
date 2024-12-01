# Advent of Code 2024

My solutions to AoC 2024, using the [cargo-aoc](https://github.com/gobanos/cargo-aoc) library.

## Notes

### Writing

Add your day script into `lib.rs` by including `pub mod day<day_num>`, and use this template for a day:

```rust
#[aoc_generator(day<day_num>)]
pub fn input_generator(input: &str) -> <DATA TYPE> {
    todo!()
}

#[aoc(day<day_num>, part1)]
pub fn solve_part1(input: &<DATA TYPE>) -> isize {
    todo!()
}

#[aoc(day<day_num>, part2)]
pub fn solve_part2(input: &<DATA TYPE>) -> isize {
    todo!()
}
```

### Running

Get the input and run today's solution:

```bash
cargo aoc input
cargo aoc
```
