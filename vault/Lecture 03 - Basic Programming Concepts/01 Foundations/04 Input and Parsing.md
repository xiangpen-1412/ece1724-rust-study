# 04 Input and Parsing

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

## Core idea

Reading keyboard input produces text. Parsing is a separate operation that interprets that text as a chosen type. Successfully reading a line does not imply that the line contains a valid number.

The example performs three steps: read into a `String`, remove surrounding whitespace from the text presented to the parser, then parse a `u32`.

## Worked example

```rust
use std::io;

fn main() {
    let mut guess = String::new();
    println!("Enter a non-negative integer:");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Expected an integer in the u32 range");

    println!("Parsed number: {guess}");
}
```

After entering `  42  ` and pressing Enter, the final output is:

```text
Parsed number: 42
```

## What each operation does

`String::new()` creates an empty text buffer. `read_line(&mut guess)` may modify it, so this binding must be mutable. The method appends input to the buffer, including the line ending when present.

`read_line` returns `Result<usize, io::Error>`. Success contains the number of bytes read, not the number the user typed. The first `expect` accepts a successful read or panics on an I/O error. This example does not use the returned byte count.

`trim()` removes leading and trailing whitespace from the text used by the following operation. It does not edit the original buffer in place, and it does not remove spaces between digits.

`parse()` needs a target type. The annotation `let guess: u32` supplies it. Writing `parse::<u32>()` is another way to specify that target. Parsing returns another `Result`; the second `expect` extracts the number or panics if parsing fails.

The final `let` shadows the text binding with a numeric binding; see [[03 Scope and Shadowing]].

## Pitfalls

For `u32`, the accepted numeric range is `0..=4_294_967_295`. Inputs such as `abc`, `-1`, an empty trimmed line, or `4294967296` fail. Whitespace around `42` is harmless after trimming; `4 2` is not a valid integer.

The message passed to `expect` does not choose the parsing type. The type annotation or parser type argument does that. This reference intentionally panics on invalid input; it does not repeatedly prompt until input becomes valid.

## Reference code

Source: `src/lectures/lec3/04_input_parsing.rs`

```bash
cargo run --bin lec3_input_parsing
```
