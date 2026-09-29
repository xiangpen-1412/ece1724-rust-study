# 02 Move and Clone

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

Assigning a String transfers ownership. The old binding cannot read the moved value; it does not become an empty string or a null pointer. The new owner takes responsibility for the same buffer, preventing duplicate cleanup.

```rust
fn main() {
    let original = String::from("draft");
    let mut revised = original.clone();
    revised.push_str(" v2");

    let saved = original;
    println!("{saved} | {revised}"); // draft | draft v2
    // println!("{original}"); // Error: moved value.
}
```

**Move transfers a value; String's clone copies its text into independent storage.** Cloning has allocation/copying costs, so use it when two independent strings are actually needed. Other types define their own clone behavior. A move describes language semantics, not a guaranteed machine-level copying operation. Formatting with `println!` borrows its String arguments and leaves them usable.

## Reinitialization and replacement

```rust
let mut current = String::from("old");
let saved = current;
current = String::from("new");
println!("{current}"); // new
current = String::from("replacement"); // Drops "new".
println!("{saved} | {current}"); // old | replacement
```

Reinitializing `current` does not reclaim `saved`'s String. In contrast, replacing a value that has not moved drops that previous value. Cleanup can therefore occur before the closing brace.

Mutability and ownership are separate: an immutable String binding can move into a new `mut` binding. Making the old binding mutable would not make String Copy. See [[03 Copy and Compound Values]].

## Code reference

Source: `src/lectures/lec4/02_move_and_clone.rs`

Run: `cargo run --bin lec4_move_and_clone`

[Rust Book: Move and clone](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)
