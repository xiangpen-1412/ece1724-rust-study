# Lecture 3 - Basic Programming Concepts

[[00 Start Here|Vault home]] · [[Lecture 03 - Basic Programming Concepts/90 Code Map|Code map]] · [[Lecture 03 - Basic Programming Concepts/99 Review Checklist|Review checklist]]

This lecture covers bindings, basic types, compound values, functions, and control flow. The notes also retain the worked extensions from the study sessions: numeric overflow, conversion order, floating-point comparisons, Unicode terminology, expression types, and loop boundaries.

## Topics

| Order | Topic | Main question |
| --- | --- | --- |
| 01    | [[01 Variables and Mutability]] | What can a binding change, and what remains fixed?      |
| 02    | [[02 Constants and Static]]     | How do `const`, `static`, and ordinary bindings differ? |
| 03    | [[03 Scope and Shadowing]]      | Which binding does a name refer to in this block?       |
| 04    | [[04 Input and Parsing]]        | How does text input become a typed value?               |
| 05    | [[05 Type Inference]]           | How do annotations and later uses constrain a type?     |
| 06 | [[06 Integer Types and Overflow]] | Which values fit, and what happens outside the range? |
| 07 | [[07 Numeric Operations]] | How do division, remainders, and floating-point arithmetic behave? |
| 08 | [[08 Type Conversions]] | When does conversion lose information or change a result? |
| 09 | [[09 Booleans and Short Circuiting]] | Which conditions and operands are actually evaluated? |
| 10 | [[10 Characters and Unicode]] | What is the difference between a byte, a scalar value, and visible text? |
| 11 | [[11 Tuples]] | How do fixed fields, destructuring, and unit work? |
| 12 | [[12 Arrays]] | How do element types, lengths, and bounds affect access? |
| 13 | [[13 Functions and Expressions]] | What value does each block or function return? |
| 14 | [[14 If Expressions]] | Which branch runs, and which result type is required? |
| 15 | [[15 Loop Expressions]] | How can a loop finish with a value? |
| 16 | [[16 While Loops]] | Will the condition become false, including after `continue`? |
| 17 | [[17 For Loops and Labels]] | How do iteration, ranges, indices, and nested exits work? |

## Connections to notice

- Mutability permits changing a value; shadowing creates a new binding. Neither removes type checking.
- A type annotation constrains inference; a cast performs a conversion. They are different operations.
- A block's final expression determines its value. A trailing semicolon often changes that value to `()`.
- `if` and `loop` can produce values, so control flow and types must be considered together.
- Array indices and integer operations need boundary reasoning, even when the code compiles.

These pages focus on Lecture 3. A brief warning about non-`Copy` values prevents overgeneralizing the numeric examples; ownership and borrowing are a later lecture.

## Course reference

[Lecture 3 slides](https://iqua.ece.toronto.edu/baochun/ece1724/slides/lecture3.html) · [The Rust Book, Chapter 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

The lecture's final reading assignment is Rust Book Chapter 3. Use [[Lecture 03 - Basic Programming Concepts/99 Review Checklist|99 Review Checklist]] after completing the topic notes.
