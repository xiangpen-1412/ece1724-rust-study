# 12 String Operations

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

## Build and combine owned text

`String::new()` creates an empty String. `String::from("text")` and `"text".to_string()` create owned text from a literal. A mutable String can grow while retaining valid UTF-8.

```rust
fn main() {
    let mut left = String::new();
    left.push_str("red");
    left.push('!');
    let right = "blue".to_string();

    let preview = format!("{left}/{right}");
    let joined = left + &right;
    println!("{preview} | {joined} | {right}");
    // red!/blue | red!blue | blue
    // println!("{left}"); // Error: + consumed left.
}
```

Their ownership effects differ:

| Operation | Effect |
| --- | --- |
| `text.push_str(piece)` | Borrow a string slice and append its contents |
| `text.push(character)` | Append one `char` |
| `left + &right` | Consume left, borrow right, return String |
| `format!("{left}/{right}")` | Borrow these inputs and create a new String |

## Read the ownership contract

For String addition, the left operand becomes the method's owned `self`; the appended operand is borrowed as `&str`. This explains why `left` becomes unavailable while `right` remains usable. The left binding does not need `mut` for ownership to be consumed.

`push_str(&right)` also leaves `right` usable. Appending its contents changes the destination, not the source. `push` takes a single-quoted character such as `'!'`; `push_str` takes text such as `"!"`.

Formatting combines several pieces while keeping the original Strings usable.

## Code reference

No dedicated practice file exists yet. Revisit basic creation and appending in `src/lectures/lec4/01_string_and_scope.rs`.

[Rust Book: Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
