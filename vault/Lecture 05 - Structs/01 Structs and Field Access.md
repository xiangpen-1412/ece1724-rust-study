# 01 Structs and Field Access

[[Lecture 05 Overview|Lecture 5 overview]] · [[Lecture 05 - Structs/90 Code Map|Code map]]

## Define a type, then create a value

A struct groups related values under named fields. The definition describes a type; an instance supplies the actual data. Without update syntax, every field must appear exactly once. Initializer order can differ from declaration order because the field names identify the values.

```rust
struct Player {
    name: String,
    score: u32,
    active: bool,
}

fn main() {
    let mut player = Player {
        active: true,
        name: String::from("Mina"),
        score: 10,
    };

    player.score += 5;
    player.name.push('!');
    println!("{}: {} ({})", player.name, player.score, player.active);
    // Mina!: 15 (true)

    let score = player.score;
    let name = &player.name;
    println!("{name}: {score}"); // Mina!: 15
}
```

## Access, mutation, and ownership

Dot notation accesses a field. In this example, `mut` on the binding permits changing its fields; `mut` is not written into individual field declarations.

Access alone does not always move a field: its use matters. `let score = player.score` copies the integer; `let name = &player.name` borrows the String; `let name = player.name` would move that String. Printing a field borrows it, so printing the name above does not remove it.

A `String` field owns text. A literal such as `"Mina"` has type `&str`, so it cannot directly fill this field without conversion. Borrowed fields are possible, but require an explicit lifetime design; keep owned Strings for these exercises.

**Check:** Does removing `mut` still allow construction? **Answer:** Yes, but the two subsequent modifications fail to compile. Reading fields remains valid.

## Code reference

Source: `src/lectures/lec5/01_structs_and_fields.rs`

Run: `cargo run --bin lec5_structs_and_fields`

[Rust Book: Defining structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) · [Reference: Field access](https://doc.rust-lang.org/reference/expressions/field-expr.html)
