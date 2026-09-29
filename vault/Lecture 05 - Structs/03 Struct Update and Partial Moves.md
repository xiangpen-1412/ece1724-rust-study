# 03 Struct Update and Partial Moves

[[Lecture 05 Overview|Lecture 5 overview]] · [[Lecture 05 - Structs/90 Code Map|Code map]]

## Fill remaining fields from another instance

`Type { changed_field: value, ..old }` constructs a new instance. `..old` comes last and supplies only fields not explicitly provided. It does not mutate `old` or automatically clone its data.

```rust
struct Account {
    name: String,
    email: String,
    visits: u32,
    active: bool,
}

fn main() {
    let first = Account {
        name: String::from("Mina"),
        email: String::from("old@example.com"),
        visits: 3,
        active: true,
    };
    let second = Account {
        email: String::from("new@example.com"),
        ..first
    };

    println!("{} {} {}", first.email, first.visits, first.active);
    // old@example.com 3 true
    println!("{} {}", second.name, second.email);
    // Mina new@example.com
    // println!("{}", first.name); // Error: this field moved.
    // let original = first; // Error: first is partially moved.

    let third = Account {
        name: String::from("Tao"),
        email: String::from("third@example.com"),
        ..second
    };
    let still_whole = second;
    println!("{} {}", still_whole.name, third.visits); // Mina 3
}
```

## Audit fields individually

| Field used to construct `second` | Effect on `first` |
| --- | --- |
| `name` | String moves; this field becomes unavailable |
| `email` | Explicit replacement; original remains available |
| `visits`, `active` | Copy; originals remain available |

This is a partial move: untouched fields remain usable, but the incomplete original cannot be passed or assigned as a whole.

Constructing `third` supplies both Strings explicitly. Only Copy fields come from `second`, so `second` remains complete; the later whole-value assignment proves this.

**Check:** Would replacing `..first` with `..first.clone()` automatically work? **Answer:** No. User-defined structs do not automatically implement Clone. Likewise, a struct whose fields are all Copy does not automatically become Copy. Field behavior and whole-struct behavior are separate questions.

## Note

1. Fields can be copied, but not the entire struct

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
	let a = Point { x: 1, y: 2 };
	let b = a;
	
	println!("{}", a.x); // error: whole point moved
}
```

## Code reference

Source: `src/lectures/lec5/03_struct_update.rs`

Run: `cargo run --bin lec5_struct_update`

[Reference: Struct update](https://doc.rust-lang.org/reference/expressions/struct-expr.html#functional-update-syntax) · [Copy documentation](https://doc.rust-lang.org/std/marker/trait.Copy.html)
