# 04 Tuple and Unit-Like Structs

[[Lecture 05 Overview|Lecture 5 overview]] · [[Lecture 05 - Structs/90 Code Map|Code map]]

## Tuple structs preserve a meaningful type name

A tuple struct has numbered fields instead of named fields. Declare it with `struct Name(Type, ...);`, construct it with `Name(value, ...)`, and access fields with `.0`, `.1`, and so on.

```rust
struct Meters(u32);
struct Seconds(u32);
struct Ready;

fn report_distance(distance: Meters) {
    let Meters(amount) = distance;
    println!("{amount} m");
}

fn start(_signal: Ready) {
    println!("started");
}

fn main() {
    let distance = Meters(12);
    let duration = Seconds(12);
    println!("{} {}", distance.0, duration.0); // 12 12

    // report_distance(duration); // Error: Seconds is not Meters.
    report_distance(distance); // 12 m
    // println!("{}", distance.0); // Error: distance moved.

    let signal = Ready;
    start(signal); // started
}
```

`Meters` and `Seconds` are different types despite identical field types. A plain `(u32,)` is a third type. This can prevent accidentally passing a duration where a distance is required.

Destructuring includes the type name: `let Meters(amount) = distance`. The ordinary tuple pattern `let (amount,) = distance` does not match a Meters value.

## Unit-like structs still define types

`struct Ready;` declares a type without fields. `Ready` constructs a value of that type; it is distinct from the unit value `()`. Such values can identify a marker or signal without carrying extra data.

No fields does not mean “automatically Copy.” Neither Ready nor Meters has Copy in these declarations. A by-value argument moves the whole value, even though reading the integer field `distance.0` by value would copy that integer.

**Check:** Can `start(())` replace `start(signal)`? **Answer:** No. The function expects Ready, not the unit type. Equal field counts or similar appearances do not make independently defined types interchangeable.

## Code reference

Source: `src/lectures/lec5/04_tuple_and_unit_structs.rs`

Run: `cargo run --bin lec5_tuple_and_unit_structs`

[Reference: Struct forms](https://doc.rust-lang.org/reference/items/structs.html) · [Copy documentation](https://doc.rust-lang.org/std/marker/trait.Copy.html)
