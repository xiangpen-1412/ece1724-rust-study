# Lecture 4 Review Checklist

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

Use these prompts after the topic notes. First decide whether the whole program compiles; only then predict output or a possible runtime panic.

- [ ] Explain the difference between String's description and its buffer, and between length and capacity. [[01 String and Scope]]
- [ ] Trace move, clone, reinitialization, replacement, Copy, and a partial move. [[02 Move and Clone]] · [[03 Copy and Compound Values]]
- [ ] Follow ownership through arguments and returns; explain why `(text, text.len())` fails. [[04 Function Ownership]]
- [ ] Distinguish an independent returned integer from a returned reference. [[05 Shared References]]
- [ ] Explain `mut text: &String` versus `text: &mut String` and when `*` is needed. [[06 Mutable References]]
- [ ] Locate each borrow's last use; identify conflicting reads, writes, moves, and dangling returns. [[07 Borrowing Rules and Lifetimes]]
- [ ] Explain how a stored index can become stale and how a slice changes the validity checks. [[08 First Word and Stale Indices]] · [[09 String Slices]]
- [ ] Choose `&str`, String, or a slice according to the required access and lifetime. [[10 str Parameters and Borrowed Results]] · [[11 Array Slices]]
- [ ] Trace ownership through `push_str`, `+`, and `format!`. [[12 String Operations]]
- [ ] Separate byte positions, scalar values, visible characters, and valid UTF-8 boundaries. [[13 UTF-8 and String Indexing]]

## Two short checks

**1. Does this compile, and what is printed? What changes if `println!("{read}")` moves after `push`?**

```rust
let mut text = String::from("go");
let read = &text;
println!("{read}");
text.push('!');
println!("{text}");
```

**Answer:** It prints `go`, then `go!`. Moving the reference's print after `push` makes its shared borrow overlap mutation, so the whole program fails to compile and prints nothing.

**2. What happens if the commented line is enabled?**

```rust
let text = String::from("\u{4E2D}A");
println!("{}", text.len());
// println!("{}", &text[..1]);
```

**Answer:** As written, it prints `4`. With the line enabled, it still compiles, prints `4`, then panics because byte 1 lies inside the first scalar's UTF-8 encoding. By contrast, `text[0]` is a compilation error.

[Official Quiz 2 sample questions](https://iqua.ece.toronto.edu/baochun/ece1724/assignments/files/quiz2-questions.pdf) · [Official sample answers](https://iqua.ece.toronto.edu/baochun/ece1724/assignments/files/quiz2-answer-key.pdf)
