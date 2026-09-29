# 15 Loop Expressions

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

`loop` repeats its body without an automatic stopping condition. A condition inside the body may decide when to exit. Reaching the closing brace starts another iteration; a final expression alone does not exit the loop.

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
println!("{counter}, {result}"); // 10, 20
```

`break counter * 2` exits this loop and supplies its result. It does not end the surrounding function. The semicolon after the closing brace ends the outer `let` statement.

## Break values and unit

Different `break` paths targeting the same loop must provide compatible types. Integer `1` and string `"one"` are incompatible even when the condition choosing between them looks constant.

Plain `break;` supplies `()`. It cannot be mixed with an ordinary integer result for the same loop.

```rust
let mut remaining = 3;
let result = loop {
    if remaining == 0 {
        break;
    }
    remaining -= 1;
};
println!("{remaining}, {result:?}"); // 0, ()
```

Ordinary `while` and `for` loops accept `break;`, but not a direct `break value` that returns their result. Use `loop` when this value-producing pattern fits the task.

## Shadowing after a loop finishes

This example finds the first multiple of seven above twenty:

```rust
let mut counter = 7;
let counter = loop {
    if counter > 20 {
        break counter;
    }
    counter += 7;
};
println!("{counter}"); // 21
```

The new immutable `counter` does not enter scope during its own initializer. References inside the loop therefore use the old mutable binding, progressing through `7`, `14`, and `21`. Once the initializer finishes, the new binding holds the loop's result. This is shadowing, not a recursive reference to an uninitialized new variable.

For termination, identify which state changes, the exit condition, and whether every continuing path can reach that change. `continue` skips the remainder of the current iteration; `return` exits the entire function. Their targets differ from `break`.

Related: [[16 While Loops]] · [[17 For Loops and Labels]]

## Code reference

Repository path: `src/lectures/lec3/15_loop_expressions.rs`

```bash
cargo run --bin lec3_loop_expressions
```

[Rust Book: Control flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
