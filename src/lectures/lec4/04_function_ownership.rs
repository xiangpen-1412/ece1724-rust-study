fn main() {
    // Practice 04: Ownership in function arguments and return values.

    let message = String::from("ready");

    // Value used after being moved [E0382]
    // inspect(message);

    // ownership: message -> decorate -> new_message
    let new_message = decorate(message);

    // still error, cannot get ownership back
    // println!("{}", message);
    println!("{new_message}");

    let original = String::from("Q");
    let count = 2;

    let (updated, next) = revise(original.clone(), count);

    println!("{original}|{updated}|{count}|{next}");
}

// this fn takes the ownership of message
fn inspect(message: String) {
    println!("{}", message);
}

// integers are okay
fn increase(message: i32) {
    println!("{}", message);
}

fn decorate(message: String) -> String{
    println!("{}", message);
    message
}

fn describe(text: String) -> (usize, String) {
    // (text, text.len()) Error: text used after moved
    (text.len(), text)
}

fn revise(mut text: String, count: i32) -> (String, i32) {
    text.push('!');
    (text, count + 1)
}

