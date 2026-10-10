use log::error;

fn main() {
    // Practice 09: String slices, borrowing, and UTF-8 boundaries.
    let text = String::from("hello");
    let first: &str = &text[..3];
    let second: &str = &text[4..];
    let third: &str = &text[2..2];

    println!("{}", first);
    println!("{}", second);
    println!("{}", third);

    error();
}

fn error() {
    let text = String::from("hello");
    let first = &text[..3];

    // text.clear();
    println!("{first}");
}

fn test() {
    let mut text = String::from("hello");
    let first = &text[..3];

    let second= first.to_owned();
    text.clear();
}

