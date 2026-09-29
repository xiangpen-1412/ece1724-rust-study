# Lecture 5 - Structs

[[00 Start Here|Vault home]] · [[Lecture 05 - Structs/90 Code Map|Code map]] · [[Lecture 05 - Structs/99 Review Checklist|Review checklist]]

This folder covers **Rust Book Section 5.1 only**, matching Lecture 5 slides 2–14. Structs give related data a named type; the ownership rules from [[Lecture 04 Overview|Lecture 4]] still apply to each field. Read the four numbered topics in order, then use the review questions with their answers below.

## Topics

| Order | Topic | What this note covers |
| --- | --- | --- |
| 01 | [[01 Structs and Field Access]] | Definition versus instance, named fields, initialization order, dot access, mutability, owned String fields |
| 02 | [[02 Builder Functions and Field Shorthand]] | Ordinary functions returning structs, tail expressions, field shorthand, ownership of arguments |
| 03 | [[03 Struct Update and Partial Moves]] | `..source`, omitted fields, move versus Copy, usable fields after a partial move |
| 04 | [[04 Tuple and Unit-Like Structs]] | Positional fields, distinct named types, destructuring, fieldless structs |

## Scope and practice

Topics 01, 02, 03, and 04 map to slides 2–9, 10–11, 12, and 13–14 respectively. Each has a matching numbered practice file in `src/lectures/lec5/`. The files start with an empty `main` and a short prompt so you can write the implementation yourself; exact paths and run targets are in the code map.

The next slide begins the Section 5.2 rectangle example. This folder stops before that material: printing custom structs, methods, `impl`, and enums are outside this review. Reference fields are mentioned only to explain why these examples use owned `String` values.

[Lecture 5 slides](https://iqua.ece.toronto.edu/baochun/ece1724/slides/lecture5.html) · [Rust Book Section 5.1](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
