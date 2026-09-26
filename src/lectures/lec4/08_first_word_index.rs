fn main() {
    let mut text = String::from("hello world");
    let saved_end = first_word_end(&text);
    println!("Text: {text:?}; saved end: {saved_end}"); // 5

    // A returned index does not keep the text borrowed.
    text.clear();
    println!(
        "After clear: length = {}; saved end = {saved_end}",
        text.len()
    );
    // let stale_word = &text[..saved_end]; // Panics: 5 exceeds the current length.

    text.push_str("hi all");
    let current_end = first_word_end(&text);
    println!("New text: {text:?}; old end: {saved_end}; current end: {current_end}");
    // An in-bounds index can still be stale: "hi al" is not the first word.
    println!(
        "Stale range: {:?}; correct range: {:?}",
        &text[..saved_end],
        &text[..current_end]
    );

    let no_space = String::from("hello");
    let empty = String::new();
    println!("No space: {}", first_word_end(&no_space)); // 5
    println!("Empty string: {}", first_word_end(&empty)); // 0
}

fn first_word_end(text: &String) -> usize {
    // enumerate gives (byte index, &u8); &byte copies the referenced byte.
    for (i, &byte) in text.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return i;
        }
    }
    text.len()
}
