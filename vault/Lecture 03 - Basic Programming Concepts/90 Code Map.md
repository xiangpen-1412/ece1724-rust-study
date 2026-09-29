# Lecture 3 Code Map

[[00 Start Here|Vault home]] · [[Lecture 03 Overview|Lecture 3 overview]] · [[Lecture 03 - Basic Programming Concepts/99 Review Checklist|Review checklist]]

Every topic below maps to one existing runnable example. All source files are under `src/lectures/lec3/`, relative to the repository root.

## Source files and run targets

| Topic | Source on GitHub | Cargo target |
| --- | --- | --- |
| [[01 Variables and Mutability]] | [01_variables_and_mutability.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/01_variables_and_mutability.rs) | `lec3_variables` |
| [[02 Constants and Static]] | [02_constants_and_static.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/02_constants_and_static.rs) | `lec3_constants_static` |
| [[03 Scope and Shadowing]] | [03_scope_and_shadowing.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/03_scope_and_shadowing.rs) | `lec3_scope_shadowing` |
| [[04 Input and Parsing]] | [04_input_parsing.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/04_input_parsing.rs) | `lec3_input_parsing` |
| [[05 Type Inference]] | [05_type_inference.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/05_type_inference.rs) | `lec3_type_inference` |
| [[06 Integer Types and Overflow]] | [06_integer_types.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/06_integer_types.rs) | `lec3_integer_types` |
| [[07 Numeric Operations]] | [07_numeric_operations.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/07_numeric_operations.rs) | `lec3_numeric_operations` |
| [[08 Type Conversions]] | [08_type_conversions.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/08_type_conversions.rs) | `lec3_type_conversions` |
| [[09 Booleans and Short Circuiting]] | [09_booleans.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/09_booleans.rs) | `lec3_booleans` |
| [[10 Characters and Unicode]] | [10_characters.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/10_characters.rs) | `lec3_characters` |
| [[11 Tuples]] | [11_tuples.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/11_tuples.rs) | `lec3_tuples` |
| [[12 Arrays]] | [12_arrays.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/12_arrays.rs) | `lec3_arrays` |
| [[13 Functions and Expressions]] | [13_functions.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/13_functions.rs) | `lec3_functions` |
| [[14 If Expressions]] | [14_if_expressions.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/14_if_expressions.rs) | `lec3_if_expressions` |
| [[15 Loop Expressions]] | [15_loop_expressions.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/15_loop_expressions.rs) | `lec3_loop_expressions` |
| [[16 While Loops]] | [16_while_loops.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/16_while_loops.rs) | `lec3_while_loops` |
| [[17 For Loops and Labels]] | [17_for_loops.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/17_for_loops.rs) | `lec3_for_loops` |

## Run locally

Open the repository in RustRover, navigate to the matching source file, and use the run button next to `fn main()`. Each file has its own target registered in `Cargo.toml`.

From the repository root, replace the target in this example with a target from the table:

```bash
cargo run --bin lec3_variables
```

Arguments after `--` belong to the example program. For example:

```bash
cargo run --bin lec3_arrays -- 2
```

Normal numeric examples use the development profile. To compare integer-overflow behavior with the project's release profile, use `--release` as explained in [[06 Integer Types and Overflow]].

## Portable links

The GitHub links use the repository's `main` branch and do not depend on where either computer stores its checkout. They show committed code; unsaved or uncommitted local experiments will not appear there. For offline access, use the repository-relative paths and RustRover.

Code stays in its existing source directory. The vault does not contain duplicate `.rs` files that could drift away from your working examples.
