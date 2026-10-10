fn main() {
    // Practice 02: Move, clone, and reinitialization.
    let a = String::from("ticket");
    let _b = a;
    // Value used after being moved[E0382]
    // println!("{a}, {b}");

    let original = String::from("draft");
    let mut edited = original;
    edited.push_str(" v2");
    println!("Edited: {edited:?}");

    let mut status = String::from("waiting");
    status = String::from("done");

    let mut current = String::from("old");
    let saved = current;
    // Value used after being moved[E0382]
    // println!("{}", current);

    // reassign but not reuse after move
    // 不能读取已经移走的值；合法地重新初始化以后，可以使用新值。
    current = String::from("new");

    // clone can avoid moving
    let mut original = String::from("report");
    let mut revised = original.clone();
    original.push_str(" Revised");
    revised.push_str(" v2");

    println!("{}", original);
    println!("{}", revised);

    // integers (bool & char) are not affected
    let mut i:i32 = 0;
    let mut j = i;
    i += 1;

    let first: &str = "second";
    let second = first;
    println!("{}", first);
}
