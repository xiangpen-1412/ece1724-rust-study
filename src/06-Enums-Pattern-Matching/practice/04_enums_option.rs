/*
04 - Enums and Option (Book 6.1)

Question 1 - Variant shapes and String moves

Define Message with Debug but without Copy or Clone:
- Quit: no payload.
- Move: named fields x: i32 and y: i32.
- Write: one String.
- Color: three u8 values.

Judge these snippets independently:
    let a = Message::Move(3, 4);                  // A
    let b = Message::Write("hello");             // B
    let c = [Message::Quit, Message::Color(1, 2, 3)]; // C

    let text = String::from("hello");
    let message = Message::Write(text);
    println!("{text}");                          // D

Add an inspect method that prints the current message using Debug.
Choose a receiver that allows two calls and later use of the entire message.
Repair A, B, and D while preserving their intended data. Explain why C's
elements can belong to one array.

Question 2 - Option type checking is not a null check

Find all independent compilation problems:
    let missing = None;
    let n: u8 = 5;
    let maybe: Option<u8> = Some(7);
    let total = n + maybe;
    let mixed = [Some(7u8), Some('r')];

Annotate missing with a type meaning 'an optional String'.
Define Reading with a sensor name, an optional u8 temperature, and an online
flag. Construct and Debug-print two online readings: measured zero and no
measurement yet. Explain why zero cannot stand for missing data and why
Option<u8> and u8 are different types.
Do not use unwrap or expect. Explain the error in total; its full repair
will use match in the next lesson.
*/

// TODO: Define the required types and functions yourself.

fn main() {
    // Question 1: Record predictions, then write and verify your code.
    // TODO: Add your work here.

    // Question 2: Record predictions and explain your implementation.
    // TODO: Add your work here.
}
