fn main() {
    let mut text = String::from("hello world");
    let end = first_word_end(&text);

    text.clear();
    text.push_str("hi all");

    println!("{}", &text[..end]);
}

fn first_word_end(text: &String) -> usize {
    for (index, &byte) in text.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return index;
        }
    }
    text.len()
}