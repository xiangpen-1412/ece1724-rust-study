# 04 Function Ownership

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

Passing an argument follows its type: a String parameter receives ownership, while an i32 parameter receives a copy. This depends on the signature, even when the function body only prints. Returning String transfers ownership to the caller; ignoring the return value does not restore the original binding.

```rust
fn describe(mut text: String) -> (String, usize) {
    text.push('!');
    let length = text.len();
    (text, length)
}

fn main() {
    let original = String::from("go");
    let (updated, length) = describe(original.clone());
    println!("{original} | {updated} | {length}"); // go | go! | 3
}
```

The clone is moved into the function, modified, and moved back out. The original String remains unchanged. Without `.clone()`, the final use of `original` would fail to compile. A returned String is not dropped with the function's local parameter because ownership has already left.

## Order and control flow matter

Writing `(text, text.len())` is invalid: tuple elements are evaluated left to right, so the String moves before the length is read. Compute length first as above. `(text.len(), text)` also works with return type `(usize, String)`.

`let text = transform(text);` can receive ownership using a new shadowing binding. Simply calling `transform(text);` discards its returned value at the end of the statement.

If one branch moves a String, a later use must be valid on every path that reaches it. Returning immediately from the consuming branch can make a later use on the remaining path legal. Use borrowing when the function only needs temporary access; see [[05 Shared References]].

## Code reference

Source: `src/lectures/lec4/04_function_ownership.rs`

Run: `cargo run --bin lec4_function_ownership`

[Rust Book: Ownership and functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)
