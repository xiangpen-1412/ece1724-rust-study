fn main() {
    let mut title = String::from("Lecture 4");
    append_suffix(&mut title);
    println!("Updated original: {title}");

    let mut attempts = 2;
    increment(&mut attempts);
    println!("Updated number: {attempts}"); // 3

    // Method calls automatically dereference the receiver.
    let edit = &mut title;
    edit.push('!');
    println!("After method call: {title}");

    // TODO: Write push_tag(text: &mut String) to append " [checked]"; call it here.
}

// The referent is mutable; the parameter binding need not be.
fn append_suffix(text: &mut String) {
    text.push_str(" notes");
}

fn increment(number: &mut i32) {
    *number += 1; // Change the referenced integer.
}

// A shared reference does not permit this mutation.
// fn edit_shared(text: &String) {
//     text.push_str("!"); // E0596
// }
