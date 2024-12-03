## About

Solutions to the Advent of Code problems written in Rust and tuned for performance.

"Extremely fast" is a wild
overstatement&mdash;but regardless, it's the goal.

## Performance

Benchmarks are measured using the built-in `cargo bench` tool run on
an Apple M4 Pro with 24 GB of RAM.

| Year          | Benchmark (ms) |
|---------------|---------------:|
| [2024](#2024) |  (in progress) |

## 2024

| Day | Problem                                                   | Solution                        | Benchmark (μs) |
|-----|-----------------------------------------------------------|---------------------------------|---------------:|
| 1   | [Historian Hysteria](https://adventofcode.com/2024/day/1) | [Source](src/year2024/day01.rs) |             16 |
| 2   | [Red-Nosed Reports](https://adventofcode.com/2024/day/2)  | [Source](src/year2024/day02.rs) |             42 |
| 3   | [Mull It Over](https://adventofcode.com/2024/day/3)       | [Source](src/year2024/day03.rs) |            133 |