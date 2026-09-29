# Lecture 5 Review Checklist

[[Lecture 05 Overview|Lecture 5 overview]] · [[Lecture 05 - Structs/90 Code Map|Code map]]

Scope: Section 5.1 only. These are original practice questions, not copied official quiz questions. Decide whether the whole program compiles before predicting output. Answers follow each question.

- [ ] Define a type, construct an instance with every required field, and access fields by name. [[01 Structs and Field Access]]
- [ ] Explain why modifying a field requires a mutable instance and why a String field needs an owned value.
- [ ] Write a returning function and expand field shorthand without changing ownership. [[02 Builder Functions and Field Shorthand]]
- [ ] For `..old`, identify exactly which fields move, copy, or remain untouched. [[03 Struct Update and Partial Moves]]
- [ ] Distinguish copying omitted fields from moving an entire user-defined struct.
- [ ] Construct and destructure a tuple struct; explain why equal field types do not make two named types interchangeable. [[04 Tuple and Unit-Like Structs]]
- [ ] Define and instantiate a unit-like struct without confusing it with `()`.

## 1. A builder and the caller

```rust
struct Label { text: String }

fn build(text: String) -> Label {
    Label { text }
}

fn main() {
    let text = String::from("sensor");
    let label = build(text);
    println!("{}", label.text);
    // println!("{text}");
}
```

**Answer:** It prints `sensor`. Enabling the last line causes a compile error: the argument moved into the function and then into the field. Shorthand does not clone. Passing `text.clone()` instead preserves the caller's original String.

## 2. Which parts of the original remain usable?

```rust
struct Account { name: String, email: String, visits: u32 }

fn main() {
    let old = Account {
        name: String::from("Ada"),
        email: String::from("old@example.com"),
        visits: 3,
    };
    let new = Account {
        email: String::from("new@example.com"),
        ..old
    };
    println!("{} {} {}", new.name, old.email, old.visits);
    // println!("{}", old.name);
}
```

**Answer:** It prints `Ada old@example.com 3`. `name` moved; `visits` copied; `old.email` was not used to build `new`, so it remains available. Enabling the last line fails. If the new instance explicitly supplies both String fields, only `visits` is copied from `old`, leaving the entire original usable.

## 3. Same fields, same type?

```rust
struct Meters(u32);
struct Seconds(u32);
struct Ready;

fn distance(value: Meters) -> u32 { value.0 }

fn main() {
    let state = Ready;
    println!("{}", distance(Meters(8)));
    // println!("{}", distance(Seconds(8)));
    // let unit: () = state;
}
```

**Answer:** As written, it prints `8`. Either commented line causes a type mismatch when enabled: `Seconds` is not `Meters`, and `Ready` is not `()`. A type's name establishes its identity; matching fields or having no fields does not erase it.

[Official Quiz 2 sample questions](https://iqua.ece.toronto.edu/baochun/ece1724/assignments/files/quiz2-questions.pdf) · [Official sample answers](https://iqua.ece.toronto.edu/baochun/ece1724/assignments/files/quiz2-answer-key.pdf)
