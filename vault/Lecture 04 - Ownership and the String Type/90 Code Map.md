# Lecture 4 Code Map

[[00 Start Here|Vault home]] · [[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/99 Review Checklist|Review checklist]]

Practice code stays in `src/lectures/lec4/`, relative to the repository root. This map points to existing files; it does not replace your implementations or create duplicate Rust sources inside the vault.

| Note | Existing source | Cargo target |
| --- | --- | --- |
| [[01 String and Scope]] | [01_string_and_scope.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/01_string_and_scope.rs) | `lec4_string_and_scope` |
| [[02 Move and Clone]] | [02_move_and_clone.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/02_move_and_clone.rs) | `lec4_move_and_clone` |
| [[03 Copy and Compound Values]] | [03_copy_and_compound_values.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/03_copy_and_compound_values.rs) | `lec4_copy_and_compound_values` |
| [[04 Function Ownership]] | [04_function_ownership.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/04_function_ownership.rs) | `lec4_function_ownership` |
| [[05 Shared References]] | [05_shared_references.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/05_shared_references.rs) | `lec4_shared_references` |
| [[06 Mutable References]] | [06_mutable_references.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/06_mutable_references.rs) | `lec4_mutable_references` |
| [[07 Borrowing Rules and Lifetimes]] | [07_borrowing_rules.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/07_borrowing_rules.rs) | `lec4_borrowing_rules` |
| [[08 First Word and Stale Indices]] | [08_first_word_index.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/08_first_word_index.rs) | `lec4_first_word_index` |
| [[09 String Slices]] | [09_string_slices.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/09_string_slices.rs) | `lec4_string_slices` |
| [[10 str Parameters and Borrowed Results]] | [10_str_parameters.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/10_str_parameters.rs) | `lec4_str_parameters` |
| [[11 Array Slices]] | [11_array_slices.rs](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec4/11_array_slices.rs) | `lec4_array_slices` |
| [[12 String Operations]] | `src/lectures/lec4/12_string_operations.rs` | `lec4_string_operations` |
| [[13 UTF-8 and String Indexing]] | `src/lectures/lec4/13_utf8_and_indexing.rs` | `lec4_utf8_and_indexing` |

Open the source file in RustRover and run its `main`, or use the matching target from the repository root:

```bash
cargo run --bin lec4_function_ownership
```

Statement-only snippets in the notes belong inside `main`; helper functions may be defined outside it. Enable intentional error lines one at a time, then comment them again to continue. An unused function containing invalid code still prevents compilation.

GitHub links show committed code. For your current local work, open the path in RustRover; uncommitted changes will not appear on GitHub.
