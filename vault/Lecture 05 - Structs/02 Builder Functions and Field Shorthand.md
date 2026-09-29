# 02 Builder Functions and Field Shorthand

[[Lecture 05 Overview|Lecture 5 overview]] · [[Lecture 05 - Structs/90 Code Map|Code map]]

## Return a constructed value

A builder here is an ordinary function returning a struct. It chooses initial values and packages its arguments into one owned result. This requires no new function syntax.

```rust
struct Ticket {
    title: String,
    priority: u8,
    open: bool,
}

fn build_ticket(title: String, level: u8) -> Ticket {
    Ticket {
        title,
        priority: level,
        open: true,
    }
}

fn main() {
    let title = String::from("Fix login");
    let level = 2;
    let ticket = build_ticket(title, level);

    println!("{}: {} ({})", ticket.title, ticket.priority, ticket.open);
    // Fix login: 2 (true)
    println!("{level}"); // 2
    // println!("{title}"); // Error: moved into build_ticket.
}
```

The final `Ticket { ... }` is the function's return expression. Adding a semicolon after it would discard that value, leaving an implicit `()` result where the signature requires `Ticket`.

## Shorthand does not change ownership

`title` abbreviates `title: title`: the field and local variable have the same name. `priority: level` must remain explicit because those names differ. Shorthand matches names, not just types.

Trace the String: the caller moves it into the function parameter, construction moves it into the field, and the function returns the owning Ticket. Returning a struct containing owned data does not leave a dangling reference to a local variable.

The integer argument is Copy, so `level` remains usable. Returning the Ticket does not restore the caller's moved `title` binding.

**Check:** How can the caller keep its title? **Answer:** Pass `title.clone()` when an independent owned copy is intended. Alternatively, design a builder taking `&str` and create an owned String inside it. The resulting field must still receive a String.

## Code reference

Source: `src/lectures/lec5/02_builders_and_shorthand.rs`

Run: `cargo run --bin lec5_builders_and_shorthand`

[Rust Book: Field init shorthand](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand) · [Reference: Struct expressions](https://doc.rust-lang.org/reference/expressions/struct-expr.html)
