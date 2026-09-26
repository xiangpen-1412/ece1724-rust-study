fn main() {
    let owned = String::from("hello world");

    // &String coerces to &str.
    println!("From &String: {:?}", first_word(&owned));
    println!("From as_str(): {:?}", first_word(owned.as_str()));
    println!("From literal: {:?}", first_word("rust notes"));
    println!("From subslice: {:?}", first_word(&owned[6..]));

    // Only ASCII space separates words in this function.
    let cases = [
        "",
        "single",
        " leading",
        "red  blue",
        "red\tblue",
        "你好 world",
    ];
    for text in cases {
        println!("{text:?} -> {:?}", first_word(text));
    }

    // to_owned copies the slice into an independent String.
    let mut original = String::from("saved value");
    let independent = first_word(&original).to_owned();
    original.clear();
    println!("Original: {original:?}; independent: {independent:?}");

    // TODO: Implement after_first_space(text: &str) -> &str.
    // Return the suffix after the first ASCII space, or an empty slice if absent.
}

// The result borrows the input; a local String would not outlive this call.
fn first_word(text: &str) -> &str {
    for (i, &byte) in text.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return &text[..i]; // ASCII space begins at a UTF-8 boundary.
        }
    }
    text
}
