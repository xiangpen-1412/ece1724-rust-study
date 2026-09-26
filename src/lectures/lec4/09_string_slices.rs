fn main() {
    let mut text = String::from("hello world");
    // These &str views borrow existing bytes; no String is cloned.
    let first: &str = &text[..5];
    let second = &text[6..];
    let whole = &text[..];
    let empty = &text[5..5];
    println!("First: {first:?}; second: {second:?}");
    println!("Whole: {whole:?}; empty: {empty:?}");

    // &str is Copy: both views refer to the same bytes.
    let copied = first;
    println!("Original view: {first:?}; copied view: {copied:?}");

    // All slice uses end before this mutation.
    text.clear();
    println!("Owner after clear: {text:?}");

    // String slice offsets are UTF-8 byte offsets and must be character boundaries.
    let unicode = String::from("中A");
    println!("UTF-8 byte length: {}", unicode.len()); // 4
    println!(
        "First character: {:?}; remainder: {:?}",
        &unicode[..3],
        &unicode[3..]
    );
    // println!("{}", &unicode[..1]); // Panics: byte 1 is inside a UTF-8 character.
    println!("Checked invalid boundary: {:?}", unicode.get(..1)); // None
}

// fn invalid_overlap() {
//     let mut text = String::from("hello world");
//     let first = &text[..5];
//     text.clear(); // E0502: first is used after this mutation.
//     println!("{first}");
// }
