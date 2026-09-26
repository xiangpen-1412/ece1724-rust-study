fn main() {
    // A literal is &str; String owns growable text.
    let literal: &str = "Rust";
    let mut owned = String::from(literal);
    owned.push_str(" study");
    owned.push('!');
    println!("Literal: {literal}; owned: {owned}");

    // len counts UTF-8 bytes. Capacity may exceed length.
    let unicode = String::from("中");
    println!("UTF-8 byte length: {}", unicode.len()); // 3
    println!(
        "Length: {}; capacity: {}; capacity >= length: {}",
        owned.len(),
        owned.capacity(),
        owned.capacity() >= owned.len()
    );

    {
        let temporary = String::from("temporary");
        println!("Inside scope: {temporary}");
    } // temporary is dropped here.
    // println!("{temporary}"); // Error: out of scope.

    // Move the block's result into an outer owner.
    let surviving = {
        let inner = String::from("created inside, owned outside");
        inner
    };
    println!("After the inner block: {surviving}");
    // println!("{inner}"); // Error: out of scope.

    let mut current = String::from("old text");
    println!("Before replacement: {current}");
    current = String::from("new text"); // Drops the old String.
    println!("After replacement: {current}");
}
