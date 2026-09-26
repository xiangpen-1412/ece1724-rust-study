fn main() {
    let mut text = String::from("note");

    // Shared borrows may coexist.
    let read_one = &text;
    let read_two = &text;
    println!("Shared reads: {read_one} | {read_two}");

    // NLL: shared borrows end after their last use here.
    let write = &mut text;
    write.push('!');
    println!("After shared reads: {text}");

    // Sequential mutable borrows need no extra block.
    let first = &mut text;
    first.push_str(" A");
    let second = &mut text;
    second.push_str(" B");
    println!("Sequential borrows: {text}");

    // Explicit scopes also separate borrows.
    {
        let edit = &mut text;
        edit.push_str(" C");
    }
    let edit = &mut text;
    edit.push_str(" D");
    println!("After scoped borrow: {text}");

    // Ownership keeps the returned allocation alive.
    let returned = make_message();
    println!("Owned return: {returned}");
}

fn make_message() -> String {
    let local = String::from("still valid after return");
    local
}

// Uncomment one complete helper to inspect its compiler error.

// E0499: the first mutable borrow is used after the second begins.
// fn overlapping_mutable() {
//     let mut text = String::from("note");
//     let first = &mut text;
//     let second = &mut text;
//     first.push('A');
//     second.push('B');
// }

// E0502: a shared borrow remains in use during a mutable borrow.
// fn mixed_borrows() {
//     let mut text = String::from("note");
//     let shared = &text;
//     let unique = &mut text;
//     println!("{shared}");
//     unique.push('!');
// }

// E0502: printing the owner conflicts with the later mutable-reference use.
// fn owner_access_during_mutable_borrow() {
//     let mut text = String::from("note");
//     let unique = &mut text;
//     println!("{text}");
//     unique.push('!');
// }

// E0106: no valid return lifetime; local is destroyed on return.
// Adding 'static cannot extend the local String's lifetime.
// fn dangling() -> &String {
//     let local = String::from("temporary");
//     &local
// }
