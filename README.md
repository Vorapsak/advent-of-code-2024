This is a template for a repository to do [Advent of Code](http://adventofcode.com) in Rust.

# How to Set Up

First you need to obtain your Advent of Code session token. See [this
link](https://github.com/wimglenn/advent-of-code-wim/issues/1) for instructions on how to obtain it,
then paste the part after `session=` in `~/.config/aoc/token`

Next, set-up your repository.

```bash
cargo install cargo-generate
cargo generate bobbobbio/Advent-of-Code-Template --name advent-of-code-<YEAR>

cd advent-of-code-<YEAR>
cargo advent init --year <YEAR>
git add .
git commit -m "Initial commit"
```

# Starting a New Day

```bash
cargo advent new-day
```

Without any arguments it will add the next day.

If the `--name` flag is provided, it will use that as the name of the crate instead of picking one.
If the `--day` flag is provided, it will assume it is for that day instead of the next one.

If you're starting the first day for example, you can find the code in `one/src/main.rs`.

Your puzzle input is downloaded to `one/input.txt`

# Running a Day

For example, to run the first day's code.
```bash
cargo run --package one < one/input.txt
```

# Submitting an Answer

```bash
cargo advent submit
```
Without any arguments, it will submit the next unsubmitted answer.

If you want to submit a specific answer, do it like this

```bash
cargo advent submit --name one --part 1
cargo advent submit --name one --part 2
```

Or like this
```bash
cargo advent submit --day 1 --part 1
cargo advent submit --day 1 --part 2
```

# Enabling the Tests
Once you have the correct answer for a problem, add the answer to the `harness!` macro to have a
test generated.

```rust
harness!(part_1: 1234, part_2: 5678)
```

# Tips on Parsing Input
Each day has a pre-populated `part_1` and `part_2` function. The argument to these functions is
anything which implements the
[`parser::HasParser`](https://bobbobbio.github.io/advent-of-code-support/parse/trait.HasParser.html) trait.

For parsing things which turn into lists, there is a convenient type to help with this called
[`List`](https://bobbobbio.github.io/advent-of-code-support/parse/struct.List.html).

Here are some examples

```rust
List<u32, SepBy<CommaSpace>> // parses: "1, 2, 3, 4"
List<u32, SepBy<Comma>> // parses: "1,2,3,4"
List<u32, TermWith<Comma>> // parses: "1,2,3,4,"
List<u32, TermWith<NewLine>> // parses: "1\n2\n3\n"
List<List<u32, SepBy<Comma>>, TermWith<NewLine>> // parses: "1,2,3,4\n5,6,7,8"
```

`HasParser` is also implemented for `Vec` with no separator.

You can derive `HasParser` for your own types easily

```rust
// By default it will parse variants as their name but converted to snake_case
// If you want to control how a variant parses, use the `string` attribute
#[derive(Debug, HasParser)]
enum Color {
    Red,
    Green,
    Blue,
}

// For structs it will parse the fields in order with space as a separator
// If you want to control the separator, use the `sep_by` attribute
#[derive(Debug, HasParser)]
struct Round {
    count: u64,
    color: Color,
}

// You can add expected strings to the parsing before or after a field
#[derive(Debug, HasParser)]
struct Game {
    #[parse(before = "Game ", after = ":")]
    id: u64,
    rounds: List<List<Round, SepBy<CommaSpace>>, SepBy<SemiSpace>>,
}

fn main() {
    let game: Game = parse::parse_str("\
        Game 1: 1 green, 6 red, 4 blue; 2 blue, 6 green, 7 red; 3 red, 4 blue, 6 green; 3 green\n\
        Game 2: 2 blue, 4 red, 7 green; 17 red, 3 blue, 2 green; 3 green, 14 red, 1 blue\
    ").unwrap();
}
```
