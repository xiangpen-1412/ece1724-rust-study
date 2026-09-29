# 08 Type Conversions

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

Source: `src/lectures/lec3/08_type_conversions.rs` ([GitHub](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/08_type_conversions.rs)).

Run: `cargo run --bin lec3_type_conversions`

**Type inference chooses a type; conversion creates a value of another type.** Once a variable is explicitly `u16`, assigning it to a `u8` variable does not automatically convert it, even when its current value would fit.

```rust
fn main() {
    let original: u16 = 300;
    let narrowed = original as u8;
    println!("{narrowed}");
    println!("{:.1}", (5 / 2) as f64);
    println!("{:.1}", 5 as f64 / 2.0);
}
```

Expected output:

```text
44
2.0
2.5
```

The narrowing integer cast keeps the low eight bits: `300 % 256` is 44. It does not report an out-of-range error. By contrast, the literal declaration `let narrowed: u8 = 300;` is rejected by default. These are different operations.

The division examples show when conversion happens. In the first, integer division runs before the cast. In the second, the left operand becomes floating-point before division.

**Use conversion APIs that express the intended guarantee.**

| Operation | Meaning |
| --- | --- |
| `u16::from(42_u8)` | Supported infallible conversion; result 42 |
| `u8::try_from(200_u16)` | Checked conversion; returns `Ok(200)` |
| `u8::try_from(300_u16)` | Checked conversion; returns an error |
| `300_u16 as u8` | Explicit narrowing; result 44 |

`TryFrom` returns a result that must be handled; it does not silently clamp or wrap the value. In the reference file, a `match` distinguishes success from failure.

**The rules for floating-point casts differ from integer narrowing.** Finite float-to-integer casts discard the fractional part toward zero: `3.9_f64 as i32` gives 3, while `(-3.9_f64) as i32` gives -3. Values outside the destination range saturate at the appropriate bound, and NaN converts to zero.

Thus `300.0_f64 as u8` gives 255, `(-1.0_f64) as u8` gives 0, and `f64::NAN as u8` gives 0. Do not predict float casts using the low-bit rule from integer casts.

An integer-to-float conversion can also lose precision without overflowing: `16_777_217_u32 as f32` becomes `16_777_216.0`. A successful cast does not promise preservation of the original number.

Related: [[06 Integer Types and Overflow]] · [[07 Numeric Operations]]
