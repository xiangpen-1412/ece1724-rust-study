# Lecture 3 Review Checklist

[[Lecture 03 Overview|Lecture 3 overview]] · [[Lecture 03 - Basic Programming Concepts/90 Code Map|Code map]]

Use this page for active recall. Explain a rule, predict an example, and then verify it with the matching topic note or source file. The checklist revisits taught material rather than adding another lecture.

## Foundations

- [ ] Explain why `mut` permits assignment but does not permit changing a binding's type.
- [ ] Distinguish a new shadowing binding from reassignment.
- [ ] Explain why an inner binding stops hiding an outer binding after the block ends.
- [ ] Compare `const`, immutable `static`, and ordinary `let` bindings.
- [ ] Trace `read_line`, `trim`, and `parse` from input text to a typed value.
- [ ] Distinguish an I/O failure from a parse failure.
- [ ] Explain how a later typed use can constrain an earlier numeric literal.

## Scalar types

- [ ] State the ranges of `u8` and `i8` and explain what `usize` is for.
- [ ] Choose between checked, wrapping, saturating, and overflowing arithmetic.
- [ ] Predict division and remainder for positive and negative integers.
- [ ] Explain why casting after integer division does not recover a fraction.
- [ ] Distinguish a type annotation from a conversion.
- [ ] Identify a narrowing cast that loses information.
- [ ] Explain why floating-point equality is not a universal approximation test.
- [ ] Predict whether the right operand of `&&` or `||` runs.
- [ ] Distinguish a `char`, a UTF-8 byte, and a displayed grapheme.

## Compound values and control flow

- [ ] Distinguish `(42)`, `(42,)`, and `()`.
- [ ] Explain why modifying a destructured integer does not modify its old tuple field.
- [ ] Distinguish `[3, 5]` from `[3; 5]` and explain why array length is part of its type.
- [ ] Identify valid array indices and explain `get` versus direct indexing.
- [ ] Determine the value and type of a block with and without a trailing semicolon.
- [ ] Distinguish printing a value from returning it.
- [ ] Explain why an unselected ordinary `if` branch still needs to type-check.
- [ ] Trace the first matching `else if` branch and check exact boundary values.
- [ ] Use `break value` to return a value from `loop`.
- [ ] Explain how `continue` can accidentally prevent a `while` loop from progressing.
- [ ] Distinguish exclusive, inclusive, reversed, and empty ranges.
- [ ] Explain `break`, `continue`, `return`, and a labeled outer-loop exit.

## Two combined practice tasks

1. Given `[12, 7, 19, 5]`, use `enumerate` to find the maximum and its index. Store them in a tuple and explain why an empty input would require a separate policy. Specify whether the first or last occurrence should win if the maximum is repeated.
2. Build a `[[i32; 3]; 2]` grid. Destructure a `(usize, usize)` position, check both bounds using short-circuit evaluation, and write `7` only when the position is valid. Try `(1, 2)` and `(2, 0)`.

## Record an actual mistake

When you make a mistake, add a short entry to the relevant topic note using three sentences: what you predicted, what actually happened, and which rule explains the difference. Include the smallest code fragment that demonstrates it.

For errors, distinguish compilation failures, runtime panics, and incorrect results from valid programs. They call for different kinds of reasoning.
