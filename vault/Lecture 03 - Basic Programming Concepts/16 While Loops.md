# 16 While Loops

[[Lecture 03 Overview|Lecture 3 overview]] · [[Lecture 03 - Basic Programming Concepts/90 Code Map|Code map]]

`while` checks a Boolean condition before every iteration. If that condition is initially false, its body executes zero times. Unlike `loop`, it includes the repeated condition check in its syntax.

```rust
let mut number = 3;
while number != 0 {
    println!("{number}");
    number -= 1;
}
println!("Liftoff!");
```

The output is `3`, `2`, `1`, then `Liftoff!`. Afterward `number` is zero, so another `while number > 0` loop immediately skips its body. In the repository example, the second loop's visit count therefore remains zero.

The condition must be a Boolean expression. Writing `while number` does not mean "while nonzero" in Rust.

## Continue and progress

`continue` skips the remaining statements of the current iteration and returns to the condition check. State changes placed after it do not run on that path.

```rust
let mut current = 0;
let mut odd_sum = 0;

while current < 5 {
    current += 1;
    if current % 2 == 0 {
        continue;
    }
    odd_sum += current;
}

println!("{current}, {odd_sum}"); // 5, 9
```

The update occurs before the possible `continue`. Values `2` and `4` skip addition; values `1`, `3`, and `5` contribute, giving `9`.

The condition is checked before the update: an iteration starts with `current = 4`, increments it to `5`, and adds `5`. That explains why the sum includes five despite the condition being `current < 5`.

A common faulty rearrangement checks for an even value and continues before incrementing `current`. Starting from zero, it repeatedly sees the same even value and never progresses. Check every continuing path, not only the path that reaches the bottom of the body.

## Exiting and choosing a loop

`break;` exits the current innermost loop. A `while` loop cannot directly produce an integer through `break 42;`; use [[15 Loop Expressions]] for value-producing loops. `return` exits the entire enclosing function, as explained in [[13 Functions and Expressions]].

Use `while` when repetition naturally depends on changing state. When processing every element or a known range, [[17 For Loops and Labels]] usually makes the traversal clearer and avoids a manually maintained index.

## Code reference

Repository path: `src/lectures/lec3/16_while_loops.rs`

```bash
cargo run --bin lec3_while_loops
```

[Rust Book: Conditional loops with while](https://doc.rust-lang.org/book/ch03-05-control-flow.html#conditional-loops-with-while)
