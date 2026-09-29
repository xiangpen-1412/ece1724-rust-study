# 06 Integer Types and Overflow

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

Source: `src/lectures/lec3/06_integer_types.rs` ([GitHub](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/06_integer_types.rs)).

Run: `cargo run --bin lec3_integer_types`

**An integer type determines both its range and whether negative values are allowed.** The `i` types are signed; the `u` types are unsigned. Available fixed widths include 8, 16, 32, 64, and 128 bits. For $n$ bits, the unsigned range is $0$ through $2^n - 1$; the signed range is $-2^{n-1}$ through $2^{n-1} - 1$.

| Type | Minimum | Maximum |
| --- | --- | --- |
| `u8` | 0 | 255 |
| `i8` | -128 | 127 |

`usize` and `isize` depend on the compilation target. Do not assume they are always 64-bit. Use `usize::BITS` to inspect the target width.

Literal notation changes how a number is written, not its value: `255_u16`, `0xff_u16`, and `0b1111_1111_u16` all represent 255. Underscores improve readability.

**Choose overflow behavior explicitly when it matters.**

```rust
fn main() {
    let value = 255_u8;
    println!("{:?}", value.checked_add(1));
    println!("{}", value.wrapping_add(1));
    println!("{}", value.saturating_add(1));
    println!("{:?}", value.overflowing_add(1));
}
```

Expected output:

```text
None
0
255
(0, true)
```

`checked_add` reports whether a representable result exists; `wrapping_add` wraps around; `saturating_add` stops at the boundary; `overflowing_add` returns the wrapped result and an overflow flag. For comparison, `254_u8.checked_add(1)` returns `Some(255)`.

**Ordinary arithmetic needs separate reasoning.** A runtime `u8` addition of 255 and 1 normally panics in Cargo's default development profile and wraps in its default release profile. The `overflow-checks` setting can change this behavior. These defaults are not the definition of every arithmetic operation.

An out-of-range literal such as `let value: u8 = 256;` is rejected by default during compilation. Other statically detected invalid expressions may also be rejected before execution. The source file therefore accepts a command-line number for its runtime experiment.

Integer division by zero, and signed `MIN / -1`, panic with ordinary `/`, including release builds. Their checked variants return `None`: `1_u8.checked_div(0)` and `i8::MIN.checked_div(-1)`. Do not apply the ordinary-addition wrapping rule to division.

Related: [[07 Numeric Operations]] · [[08 Type Conversions]]
