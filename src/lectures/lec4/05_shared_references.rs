fn main() {
    // &String borrows the value without taking ownership.
    let text = String::from("Rust");
    println!("First byte length: {}", byte_len(&text));
    println!("Second byte length: {}", byte_len(&text));
    println!("The caller still owns: {text}");

    // Multiple shared references may coexist.
    let first: &String = &text;
    let second: &String = &text;
    println!("Shared views: {first} / {second}");
    println!(
        "Lengths through both: {} / {}",
        byte_len(first),
        byte_len(second)
    );

    // len counts UTF-8 bytes, not characters.
    let unicode = String::from("中");
    println!("UTF-8 byte length: {}", byte_len(&unicode)); // 3

    // mut allows rebinding the shared reference.
    let other = String::from("another String");
    let mut view: &String = &text;
    println!("Before rebinding: {view}");
    view = &other;
    println!("After rebinding: {view}; original: {text}");
    // view.push('!'); // E0596: mut on the binding does not make &String mutable.
}

fn byte_len(s: &String) -> usize {
    s.len()
}

// fn append_through_shared(mut s: &String) {
//     s.push('!'); // E0596: shared access cannot mutate the String.
// }
