
fn dereference_num() {
    let number = 7;
    println!("{}", number);

    let r = &number;
    let number = *r;
    println!("{}", number);
}

fn dereference_str() {
    let text = String::from("hello");
    let reference = &text;

    // error: giving ownership of text to owned, not permitted
    // let owned = *reference;
    // *reference represents you find a string location, but it's owned by text

    let _len = (*reference).len();
    println!("{}", *reference);
    let a = (*reference).clone();
    println!("{}", a);
}

fn byte_length(text: &String) -> usize {
    text.len()
}

fn main() {
    // Practice 05: Shared references and reading without ownership transfer.
    let message = String::from("hello");

    let first = byte_length(&message);
    let second = byte_length(&message);

    println!("{message}: {first}, {second}");

    dereference_num();
    dereference_str();

    // test
    let mut text = String::from("ab");
    let length = byte_length(&text);

    text.push('c');

    // 2 abc
    println!("{length} {text}");
}
