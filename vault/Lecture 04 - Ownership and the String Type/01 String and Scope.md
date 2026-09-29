# 01 String and Scope

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

A `String` owns a growable UTF-8 buffer. Its description contains a pointer, byte length, and capacity. The buffer lives on the heap; a local String's description is commonly drawn on the stack. Stack frames follow function calls in last-in, first-out order; heap allocation is managed separately. Fixed address-growth directions are not Rust guarantees.

**Ownership ties resource cleanup to a value's lifetime.** Each value has one owner at a time. When that owner leaves scope, the value it still owns is dropped. For String, this releases its buffer. This RAII-style cleanup needs no tracing garbage collector: rules are checked during compilation, while allocation and cleanup happen at runtime.

```rust
fn main() {
    let literal = "quiz";
    {
        let mut owned = String::from(literal);
        owned.push_str(" tomorrow");
        println!("{literal} | {owned}"); // quiz | quiz tomorrow
        println!("{}", owned.len()); // 13 bytes
    } // owned is dropped here.
    println!("{literal}"); // quiz
    // println!("{owned}"); // Error: outside its scope.
}
```

The literal is `&str`, referring to text valid throughout the program. Constructing String creates separate owned text; appending does not change the literal. Other `&str` values may borrow a String rather than a literal.

`len()` counts used bytes; `capacity()` counts total buffer capacity, not spare space. Capacity is at least length; growth may reallocate without a guaranteed doubling rule. `mut` controls modification through the binding, not whether text is owned. Ownership transfer and earlier cleanup appear in [[02 Move and Clone]].

## Code reference

Source: `src/lectures/lec4/01_string_and_scope.rs`

Run: `cargo run --bin lec4_string_and_scope`

[Rust Book: Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
