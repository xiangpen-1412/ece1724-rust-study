# 12 String Operations

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

## Build and combine owned text

`String::new()` creates empty owned text. `String::from("red")` and `"red".to_string()` create owned text from a literal. `push_str` appends the contents of a borrowed `&str`; `push` appends one `char`.

```rust
fn main() {
    let mut piece = String::from("red");
    let mut left = String::new();
    left.push_str(&piece);
    left.push('!');

    let snapshot = format!("{left}/{piece}");
    piece.clear();
    let right = "blue".to_string();
    let joined = left + &right;
    println!("{snapshot} | {joined} | {right}");
    // red!/red | red!blue | blue
    // println!("{left}"); // Error: + consumed left.
}
```

Appending copies the source's bytes into the destination; it does not store a reference to the source. Clearing `piece` therefore changes neither `left` nor `snapshot`. The latter is also an independent String: `format!` borrows these inputs while constructing its result, without retaining those borrows.

## Read the ownership contract

`left + &right` takes the left String by value and borrows the appended text as `&str`. It returns a String, consumes `left`, and leaves `right` usable. The left binding does **not** need `mut` for addition; this example needs `mut` only for earlier appends. Writing `&left + &right` is not a supported replacement.

A chain such as `a + "-" + &b` consumes `a` and successive intermediate Strings. Use `format!("{a}-{b}")` when both inputs should remain usable. Neither technique requires manually cloning both inputs.

## Quick check

After `let view = &piece`, would `piece.clear(); println!("{view}");` work like the snapshot example?

**Answer:** No. `view` borrows the original String, so its later use conflicts with `clear()`. `push_str` and `format!` produce stored text that does not keep the source borrowed; creating a reference does.

## Note

1. Format creates a new string, changing old won't affect it

```rust
fn main() {
    let mut left = String::from("red");
    let right = String::from("blue");
	
	// saved will not be changed by changing left or right
    let saved = format!("{left}/{right}");

    left.push('!');
    let joined = left + &right;

    println!("{saved}");
    println!("{joined}");
    println!("{right}");
}
```

## Code reference

Practice file: `src/lectures/lec4/12_string_operations.rs`

Run: `cargo run --bin lec4_string_operations`

[Rust Book: Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
