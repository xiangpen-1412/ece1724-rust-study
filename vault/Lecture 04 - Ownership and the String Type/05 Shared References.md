# 05 Shared References

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

A shared reference accesses a value without owning it. `&text` creates the reference; `&String` names its type. The original owner keeps responsibility for cleanup. Multiple shared readers may coexist, and no text is cloned merely to create a reference.

```rust
fn byte_length(text: &String) -> usize {
    text.len()
}

fn main() {
    let mut text = String::from("ab");
    let length = byte_length(&text);
    text.push('c');
    println!("{length} {text}"); // 2 abc
}
```

The returned integer is a snapshot, not an ongoing borrow. Finishing this call leaves no returned reference requiring the String to remain unchanged. Returning a reference has different consequences; see [[10 str Parameters and Borrowed Results]].

## A reference and its target are different values

```rust
let text = String::from("note");
let first = &text;
let second = first; // Copies the shared reference.
println!("{first} {second}"); // note note
// let owned = *first; // Error: cannot move String through a shared reference.

let number = 7;
let copied = *&number; // Copies i32, not a String.
println!("{number} {copied}"); // 7 7
```

`*` dereferences a location; it does not automatically clone or acquire ownership. Formatting `*first` is allowed because formatting can borrow it, whereas `let owned = *first` requests an owned String value.

Even `mut reference: &String` cannot modify the String: `mut` permits rebinding that reference, not upgrading its access. For general read-only text parameters, prefer `&str`; the lecture introduces that after slices.

## Notes

1. Dereference Error

```Rust
fn dereference_str() {  
    let text = String::from("hello");  
    let reference = &text;  
  
    // error: giving ownership of text to owned, not permitted  
    // let owned = *reference;    
    // *reference represents you find a string location, but it's owned by text  
    let _len = (*reference).len();  
    println!("{}", *reference);  
    let a = (*reference).clone();  
    println!("{}", a);  
}
```
## Code reference

Source: `src/lectures/lec4/05_shared_references.rs`

Run: `cargo run --bin lec4_shared_references`

[Rust Book: References and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
