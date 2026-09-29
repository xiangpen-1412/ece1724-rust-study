fn main() {
    // Practice 07: Borrow conflicts, last use, and dangling references.
    test();
}
fn test() {
    let mut text = String::from("go");

    let first = &mut text;       // 1
    first.push('!');             // 2

    let second = &mut text;      // 3
    second.push('?');            // 4

    // println!("{first}");

    println!("{text}");          // 5
}
fn borrow_a() {
    let mut text = String::from("A");

    let read = &text;
    println!("{read}");

    let edit = &mut text;
    edit.push('B');

    println!("{text}");
}
// fn borrow_b() {
//     let mut text = String::from("A");
//
//     let write = &mut text;
//     write.push('B');
//
//     let read = &text;
//
//     println!("{read}");
//
//     println!("{write}");
// }

fn borrow_c() {
    let mut text = String::from("A");
    let edit = &mut text;

    println!("{text}");
    // edit.push('B');
}

fn borrow_d() {
    let text = String::from("A");
    let read = &text;

    println!("{text}");
    println!("{read}");
}

fn borrow_e() {
    let mut text = String::from("A");
    let read = &text;

    // text.push('!');
    println!("{read}");
}
