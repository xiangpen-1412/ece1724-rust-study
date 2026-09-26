fn main() {
    // Moving transfers ownership; the old binding cannot be used.
    let first = String::from("Rust");
    let second = first;
    // println!("{first}"); // E0382: first was moved.
    println!("New owner: {second}");

    // Formatting borrows the value, so second still owns it.
    println!("Same owner, printed again: {second}");

    // Cloning creates independent String contents.
    let original = String::from("Rust");
    let mut cloned = original.clone();
    cloned.push_str(" clone");
    println!("Original: {original}; clone: {cloned}");

    // Mutability belongs to the new binding.
    let fixed = String::from("editable after move");
    let mut editable = fixed;
    editable.push('!');
    println!("Mutable new owner: {editable}");

    // A moved mutable binding can receive a new value.
    let mut buffer = String::from("first value");
    let old_owner = buffer;
    buffer = String::from("second value");
    println!("Old owner: {old_owner}; reinitialized buffer: {buffer}");

    // TODO: clone a String, move the original into a new binding, then print both owners.
}
