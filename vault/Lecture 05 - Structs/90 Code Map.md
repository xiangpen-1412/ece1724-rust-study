# Lecture 5 Code Map

[[Lecture 05 Overview|Lecture 5 overview]] · [[Lecture 05 - Structs/99 Review Checklist|Review checklist]]

Paths below are relative to the repository root. These are practice shells, not completed solutions. Put type definitions and helper functions outside `main`, and write calls and experiments inside it.

| Note | Local source | Cargo target |
| --- | --- | --- |
| [[01 Structs and Field Access]] | `src/lectures/lec5/01_structs_and_fields.rs` | `lec5_structs_and_fields` |
| [[02 Builder Functions and Field Shorthand]] | `src/lectures/lec5/02_builders_and_shorthand.rs` | `lec5_builders_and_shorthand` |
| [[03 Struct Update and Partial Moves]] | `src/lectures/lec5/03_struct_update.rs` | `lec5_struct_update` |
| [[04 Tuple and Unit-Like Structs]] | `src/lectures/lec5/04_tuple_and_unit_structs.rs` | `lec5_tuple_and_unit_structs` |

Run one topic from the repository root:

```bash
cargo run --bin lec5_struct_update
```

Each target is independent: a type defined in file 01 is not automatically available in file 02. Recreate the small type in the file you are practicing. Enable one intentional compiler error at a time and predict the reason before running.

For whole-struct output, print the individual fields covered in these notes. Custom formatting support belongs to later material.
