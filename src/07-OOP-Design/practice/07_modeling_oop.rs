/*
07 - Enum modeling and OOP (Lecture 5; Book 18.1 concepts)

Finish lessons 01-06 first. The Quiz 3 announcement lists Book Chapters 5
and 6. The Book 18.1 OOP discussion completes the lecture but is not part
of those announced chapters.

Question 1 - Make invalid states unrepresentable

A design uses finished and failed booleans. Here finished means terminated
(successfully or unsuccessfully), and failed means terminated with failure.
Valid states are Pending, Succeeded with a String path, and Failed with a
String reason. List the four boolean combinations and identify the invalid one.
Design enum Download with the required states and payloads. Do not use two
booleans plus two Option<String> fields.

Construct all three states. Write a borrowing describe function with an
exhaustive match: Pending prints a waiting message, Succeeded the path,
and Failed the reason. Keep the original usable for Debug printing; no cloning.
Is 'three variants means only three possible values' correct? Explain using
String payloads. Explain OR for enum alternatives and AND for struct fields.

Question 2 - Encapsulation, composition, and receivers

Create an audio module exposing Volume with a private level: u8 in 0..=10.
Provide new() starting at zero, level(), and set_level(requested: u8) -> bool.
Accept valid values and reject invalid values without changing the old level.
Outside audio, define Player containing Volume and forwarding read/update
methods. Check initial zero, setting 7, trying 11, and the final level;
record return values and state. Temporarily try to change Volume's private
field from main and explain the access boundary.

Answer briefly without implementing a trait:
1. How do struct data and impl behavior satisfy some OOP definitions?
2. Why is Player containing Volume composition rather than inheritance?
3. How do traits and generics relate to polymorphism? Do they inherit fields?
*/

// TODO: Define the required types and functions yourself.

fn main() {
    // Question 1: Record predictions, then write and verify your code.
    // TODO: Add your work here.

    // Question 2: Record predictions and explain your implementation.
    // TODO: Add your work here.
}
