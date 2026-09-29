# 03 Scope and Shadowing

[[Lecture 03 Overview|Lecture 3 overview]] · [[Lecture 03 - Basic Programming Concepts/90 Code Map|Code map]]

## Core idea

A binding is usable within its scope. A nested block can introduce a new binding with the same name as an outer one; the inner name then hides the outer binding temporarily.

Shadowing means creating a new binding with `let`. Assignment means changing an existing binding. Distinguishing these operations explains both the values printed after a block and whether types or mutability may change.

## Worked example

```rust
fn main() {
    let score = 10;
    let score = score + 5;

    {
        let score = score * 2;
        println!("Inside: {score}");
    }

    println!("Outside: {score}");

    let mut count = 1;
    {
        count += 1;
    }
    println!("Count: {count}");
}
```

Expected output:

```text
Inside: 30
Outside: 15
Count: 2
```

The second `let score` evaluates its right side using the previous binding, then introduces a new binding holding 15. The nested declaration similarly creates an inner binding holding 30. When that block ends, the outer binding holding 15 becomes visible again. The original 10 is not restored.

The `count` block has no new `let count`. Its assignment changes the existing outer binding, so the change remains visible after the block.

## Rules and tricky cases

Each new binding chooses its own type and mutability. Shadowing can therefore change a name from text to a number:

```rust
let value = "42";
let value: u32 = value.parse().expect("Expected an integer");
println!("{}", value + 1); // 43
```

This does not change the type of an existing binding. It creates another binding. The parsing operation is explained in [[04 Input and Parsing]].

Likewise, mutability is not inherited:

```rust
let mut total = 10;
total += 5;
let total = total * 2;
// total += 1; // Error: the new binding is immutable.
```

## Pitfalls

A name created only inside a block cannot be used outside it. That is a scope error, not a request to add `mut`.

Using `let` accidentally when intending assignment can leave the outer value unchanged. Read the exact statement before predicting output: `let count = count + 1` and `count += 1` have different effects on bindings.

## Reference code

Source: `src/lectures/lec3/03_scope_and_shadowing.rs`

```bash
cargo run --bin lec3_scope_shadowing
```
