# 06 Mutable References

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

`&mut T` gives temporary exclusive access to a value, allowing mutation without taking ownership of that value. The caller still owns the result, so the function need not return the String just to keep it alive.

```rust
fn mark(text: &mut String) -> usize {
    text.push('!');
    text.len()
}

fn add_one(number: &mut i32) {
    *number += 1;
}

fn main() {
    let mut text = String::from("Rust");
    let length = mark(&mut text);
    let mut count = 2;
    add_one(&mut count);
    println!("{text} {length} {count}"); // Rust! 5 3
}
```

The caller permits mutation with `let mut`, creates the borrow with `&mut`, and the function accepts `&mut String`. The parameter binding itself does not need `mut`: mutation targets the referenced String. Methods dereference automatically; `*number` explicitly identifies the integer being updated.

| Parameter | Meaning |
| --- | --- |
| `mut text: String` | Own and modify a String |
| `mut text: &String` | Rebind a shared reference; cannot modify its String |
| `text: &mut String` | Modify the borrowed String |

`*text = String::from("reset")` through an `&mut String` replaces the caller's String and drops the old one. It does not rebind the reference.

Mutable references are not Copy. Nevertheless, calls to a function expecting `&mut T` can briefly reborrow an existing mutable reference, so `add_one(edit); add_one(edit);` can be valid. This is not evidence of Copy. The restrictions on overlapping access are covered in [[07 Borrowing Rules and Lifetimes]].

## Notes

1. Immutable variable Error:

```Rust
fn main() {
	// message needs to be mutable  
    let message = String::from("ready");  
    append_mark(&mut message);  
}  
  
fn append_mark(text: &mut String) {  
    text.push_str("Hello World");  
}
```

2. Mut as parameters

```Rust
// 1. String: Compile
fn decorate(mut text: String) -> String {  
    text.push('!');  
    text  
}

// 2. &String: Cannot Compile, only have reference, cannot change
fn wrong(mut text: &String) {
    text.push('!');
}

// 3. &mut String: Compile
fn right(text: &mut String) {
	text.push('!');
}
```
## Code reference

Source: `src/lectures/lec4/06_mutable_references.rs`

Run: `cargo run --bin lec4_mutable_references`

[Rust Book: Mutable references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)
