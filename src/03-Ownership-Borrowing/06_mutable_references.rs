fn main() {
    // Practice 06: Mutable references and changing the borrowed value.
    let mut message = String::from("ready");
    append_mark(&mut message);

    let mut text = String::from("hello");
    let len = mark(&mut text);
    println!("{} {}", text, len);
}

fn append_mark(text: &mut String) {
    text.push_str("Hello World");
}

fn decorate(mut text: String) -> String {
    text.push('!');
    text
}

fn mark(text: &mut String) -> usize {
    text.push('!');
    text.len()
}