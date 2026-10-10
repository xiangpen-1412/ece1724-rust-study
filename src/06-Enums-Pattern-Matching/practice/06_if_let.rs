/*
06 - If let and ownership in patterns (Book 6.3)

Question 1 - Same syntax, different payload types

Judge compilation and predict the complete output independently:

A.
    let configured = Some(0u8);
    if let Some(value) = configured {
        println!("value={value}");
    } else {
        println!("missing");
    }
    println!("{configured:?}");

B.
    let configured = Some(String::from("Rust"));
    if let Some(value) = configured {
        println!("value={value}");
    }
    println!("{configured:?}");

C.
    let configured = Some(String::from("Rust"));
    if let Some(_) = configured {
        println!("present");
    }
    println!("{configured:?}");

Repair B without cloning so the text can be read and the complete Option
remains usable. Would replacing _ with _value in C affect only warnings?

Question 2 - One requirement, two implementations

Write show_label(label: &Option<String>): print the text for Some, including
an empty String; print "<missing>" for None.
Implement it with if let ... else, then write a separate match version.
Test Some(String::from("")), Some(String::from("Rust")), and None.
Both versions must give the same output and leave the original Option usable.

Book 6.3 supplement: rewrite it using let ... else. The None branch should
print and return early; print the text after the binding on success.
Compare binding scope with if let and explain why else cannot simply print
and continue. Reuse the same three inputs rather than adding another exercise.

If Event gains a new variant, will an if let handling only Paste force you
to add logic like an explicitly exhaustive match? When might behavior be missed?
*/

// TODO: Define the required types and functions yourself.

fn main() {
    // Question 1: Record predictions, then write and verify your code.
    // TODO: Add your work here.

    // Question 2: Record predictions and explain your implementation.
    // TODO: Add your work here.
}
