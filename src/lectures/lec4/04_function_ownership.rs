fn main() {
    // Passing String by value moves it into the function.
    let message = String::from("hello");
    consume_string(message);
    // println!("{message}"); // E0382: message was moved.

    // i32 arguments are copied.
    let number = 7;
    print_number(number);
    println!("Caller still has: {number}");

    // Returning String transfers ownership to the caller.
    let created = make_message();
    println!("Returned String: {created}");

    let source = String::from("Rust");
    let updated = append_suffix(source);
    println!("Consumed, modified, returned: {updated}");

    // Return the owned value together with a computed result.
    let text = String::from("Rust");
    let (text, length) = return_with_length(text);
    println!("Returned tuple: {text}; byte length: {length}");
}

fn consume_string(s: String) {
    println!("Function owns: {s}");
} // s is dropped here.

fn print_number(n: i32) {
    println!("Function received a copy: {n}");
}

fn make_message() -> String {
    String::from("made inside the function")
}

fn append_suffix(mut s: String) -> String {
    s.push_str(" study");
    s
}

fn return_with_length(s: String) -> (String, usize) {
    // return (s, s.len()); // E0382: s moves before len is read.
    let length = s.len();
    (s, length)
}
