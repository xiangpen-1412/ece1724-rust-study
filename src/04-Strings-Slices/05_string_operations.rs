fn main() {
    // Practice 12: Compare push, push_str, +, and format! ownership.

}

fn test() {
    let left = String::from("red");
    let right = String::from("blue");

    let joined = left + &right;

    println!("{joined}"); // redblue
    println!("{right}");  // blue
    // println!("{left}"); // Error: moved
}
