# 07 Borrowing Rules and Lifetimes

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

For the same data, shared reads may coexist; mutable access must be exclusive. References must remain valid when used. These restrictions help safe Rust prevent data races, but do not eliminate all logical race conditions. They also apply in single-threaded code.

**Track the required use interval, not merely the closing brace.**

```rust
let mut text = String::from("A");
let read = &text;
println!("{read}"); // Last use of read.

let first = &mut text;
// println!("{read}"); // Error if enabled: overlapping shared/mutable access.
// println!("{text}"); // Error if enabled: owner read conflicts with first.
first.push('B');

let second = &mut text;
second.push('C');
// println!("{first}"); // Error if enabled: overlapping mutable borrows.
println!("{text}"); // ABC
```

Each commented line changes the last-use requirements. Non-lexical lifetimes (NLL) allow a borrow to finish before its reference binding leaves scope. Sequential mutable borrows are legal; overlapping ones are not. Reading the owner alongside shared references is permitted, but moving it while a later reference use still needs it is not.

## Returning valid data

```rust
fn make_text() -> String {
    let local = String::from("result");
    local // Moves ownership to the caller.
}

fn view(text: &String) -> &String {
    text // Refers to the caller's existing String.
}
```

A function returning `&local` for its own local String is invalid: that String would be dropped on return. Lifetime annotations cannot extend its actual lifetime. Returning ownership as above, or borrowing surviving caller data, avoids a dangling reference.

After `let r = view(&text)`, modifying `text` before a later use of `r` conflicts with its shared borrow. After that last use, modification may be allowed. Borrow errors prevent compilation; they are not runtime panics.

## Note

1. Undone borrow

```Rust
fn borrow_b() {  
    let mut text = String::from("A");  
  
    let write = &mut text;  
    write.push('B');  
      
    let read = &text;  
      
    println!("{read}");  
  
	// use mutable borrow here
    println!("{write}");  
}
```

2. Owner Itself

```Rust
fn borrow_c() {  
    let mut text = String::from("A");  
    let edit = &mut text;  
  
    println!("{text}");  
    
    // mutable borrow
    edit.push('B');  
}
```

3. Ownership moved

```Rust
let text = String::from("A");
let read = &text;

// ownership moved, read nolonger valid
let moved = text;
println!("{read}");
```

4. dangling reference: 
	1. Return address of a string that been destroyed after the execution of the function. 
	2. But we can return the String ownership itself instead of the reference

```rust
// invalid
fn make_reference() -> &String {
    let local = String::from("result");
    &local
}

// valid
fn make_string() -> String {
	let local  = String:from("result");
	local
}
```

5. Immutable borrow: cannot change text unless borrow finished

```rust
fn borrow_e() {  
    let mut text = String::from("A");  
    let read = &text;  
  
    text.push('!');  
    println!("{read}");  
}
```
## Code reference

Source: `src/lectures/lec4/07_borrowing_rules.rs`

Run: `cargo run --bin lec4_borrowing_rules`

[Rust Book: Borrowing rules](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
