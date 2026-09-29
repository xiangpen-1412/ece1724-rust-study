# Lecture 4 - Ownership and the String Type

[[00 Start Here|Vault home]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]] · [[Lecture 04 - Ownership and the String Type/99 Review Checklist|Review checklist]]

This lecture covers Rust Book **Chapter 4 and Section 8.2**: who owns data, how functions borrow it, and how slices and UTF-8 strings behave. Each numbered note is a short textbook topic with code, explanations, and selected pitfalls. All topic files sit directly in this lecture folder.

## Topics

| Order | Topic | What this note covers |
| --- | --- | --- |
| 01 | [[01 String and Scope]] | Stack/heap model, String versus literal, ownership rules, scope/drop, length/capacity |
| 02 | [[02 Move and Clone]] | Ownership transfer, independent cloning, reinitialization, replacement and cleanup |
| 03 | [[03 Copy and Compound Values]] | Copy by type, tuples/arrays, shared references, partial moves |
| 04 | [[04 Function Ownership]] | Parameter and return ownership, Copy arguments, tuple evaluation order, branch paths |
| 05 | [[05 Shared References]] | Reading without transfer, copying references, dereferencing, independent computed results |
| 06 | [[06 Mutable References]] | Updating caller data, positions of mut, integer dereferencing, brief reborrowing extension |
| 07 | [[07 Borrowing Rules and Lifetimes]] | Last-use reasoning, overlapping access, owner access, valid and dangling returns |
| 08 | [[08 First Word and Stale Indices]] | Byte scanning, enumerate patterns, and indices becoming logically stale |
| 09 | [[09 String Slices]] | Borrowed ranges, slice representation, source mutation and last use |
| 10 | [[10 str Parameters and Borrowed Results]] | Flexible text parameters, input/output borrowing, independent owned results |
| 11 | [[11 Array Slices]] | Shared/mutable slices, relative indices, element counts, updates through a view |
| 12 | [[12 String Operations]] | String construction, push/push_str, ownership in +, format! |
| 13 | [[13 UTF-8 and String Indexing]] | Bytes/scalars/graphemes, iteration, invalid indexing, slice boundaries and checked access |

All thirteen note numbers match Lecture 4 practice-file numbers. Topics 12 and 13 complete the lecture's String material and now have dedicated practice shells. See the code map for exact paths and available run targets.

## Reading order and scope

Topics 01–03 establish ownership (slides 4–24); 04–07 apply it to functions and references (25–45); 08–11 introduce slices (46–56); 12–13 cover String operations and UTF-8 (57–67). The notes distinguish compiler errors, runtime panics, and valid programs with incorrect results.

Continue with [[Lecture 05 Overview|Lecture 5: Structs]], limited to Book Section 5.1. Related background: [[Lecture 03 Overview|Lecture 3]].

[Lecture 4 slides](https://iqua.ece.toronto.edu/baochun/ece1724/slides/lecture4.html) · [Rust Book Chapter 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) · [Rust Book Section 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
