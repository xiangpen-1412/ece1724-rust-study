# 07 Numeric Operations

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

Source: `src/lectures/lec3/07_numeric_operations.rs` ([GitHub](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/07_numeric_operations.rs)).

Run: `cargo run --bin lec3_numeric_operations`

**Determine operand types before calculating the result.** Rust provides `f32` and `f64` for floating-point values. An unconstrained floating-point literal defaults to `f64`; an explicit annotation can select `f32`.

Integer division truncates toward zero. This is not the same as rounding downward for negative numbers. A nonzero integer remainder has the dividend's sign.

```rust
fn main() {
    println!("{} {}", 7 / 3, -7 / 3);
    println!("{} {} {}", -7 % 3, 7 % -3, -7 % -3);
    println!("{:.3}", 7.0 / 3.0);
}
```

Expected output:

```text
2 -2
-1 1 -1
2.333
```

Check integer results with `dividend = quotient * divisor + remainder`. For example, `-7 = (-2) * 3 + (-1)`. The three displayed decimal places above are output formatting, not a guarantee that the stored quotient has only three decimal places.

**Conversion order changes the calculation.** `(7 / 3) as f64` is `2.0`: integer division discarded the fraction first. `7_f64 / 3.0` performs floating-point division. Converting afterward cannot recover information already lost.

**Floating-point arithmetic approximates many decimal fractions.**

```rust
fn main() {
    let actual = 0.1_f64 + 0.2;
    let expected = 0.3_f64;
    let error = (actual - expected).abs();
    println!("{}", actual == expected);
    println!("{}", error <= 1e-12);
}
```

Expected output is `false`, then `true`. The first comparison requires exact equality of the resulting floating-point values. The second permits a small absolute error for this example.

A tolerance must fit the problem's scale and accuracy requirements. `f64::EPSILON` describes the spacing immediately above `1.0`; it is not a universal acceptable error. A fixed absolute tolerance suitable near zero may be unsuitable for very large quantities. Exact equality is still meaningful when exact equality is what the calculation requires.

**Mixed numeric types are not automatically promoted.** An `i32` and an `f64` cannot simply be added. Neither can an `f32` and an `f64`. For these combinations, `f64::from(integer)` or `f64::from(small_float)` supplies a supported explicit conversion before addition.

Related: [[06 Integer Types and Overflow]] · [[08 Type Conversions]]
