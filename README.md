# 🎄 Advent of Code 2023

Solutions for [Advent of Code](https://adventofcode.com/) in
[Rust](https://www.rust-lang.org/), using the [repository
template](https://github.com/fspoettel/advent-of-code-rust) by [Felix
Spöttel](https://github.com/fspoettel).

<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2023/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2023/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2023/day/3) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `47.9µs` | `566.7µs` |
| [Day 2](./src/bin/02.rs) | `38.6µs` | `38.4µs` |
| [Day 3](./src/bin/03.rs) | `437.4µs` | `422.8µs` |
| [Day 4](./src/bin/04.rs) | `189.5µs` | `-` |

**Total: 1.74ms**
<!--- benchmarking table --->

---

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>
```

Individual solutions live in the `./src/bin/` directory as separate binaries.
_Inputs_ and _examples_ live in the the `./data` directory.

### Download input & description for a day

```sh
# example: `cargo download 1`
cargo download <day>
```

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>
```

The `solve` command runs your solution against real puzzle inputs. To run an
optimized build of your code, append the `--release` flag as with any other rust
program.

By default, `solve` executes your code once and shows the execution time. If you
append the `--time` flag to the command, the runner will run your code between
`10` and `10.000` times (depending on execution time of first execution) and
print the average execution time.

For example, running a benchmarked, optimized execution of day 1 would look like
`cargo solve 1 --release --time`. Displayed _timings_ show the raw execution
time of your solution without overhead like file reads.

#### Submitting solutions

In order to submit part of a solution for checking, append the `--submit <part>`
option to the `solve` command.

### Run all solutions

```sh
cargo all
```

This runs all solutions sequentially and prints output to the command-line. Same
as for the `solve` command, the `--release` flag runs an optimized build.

#### Update readme benchmarks

The template can output a table with solution times to your readme. In order to
generate a benchmarking table, run `cargo all --release --time`. If everything
goes well, the command will output "_Successfully updated README with
benchmarks._" after the execution finishes and the readme will be updated.

Please note that these are not "scientific" benchmarks, understand them as a fun
approximation. 😉 Timings, especially in the microseconds range, might change a
bit between invocations.

### Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin
01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01
part_one`.

### Read puzzle description in terminal

```sh
# example: `cargo read 1`
cargo read <day>
```
