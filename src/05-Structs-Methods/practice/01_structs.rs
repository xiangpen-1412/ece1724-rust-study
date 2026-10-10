/*
01 - Structs, fields, and struct update (Book 5.1)

Work through the lesson before attempting these two review questions.
Predict first, write your own code, and then run it. Snippets are questions,
not reference solutions. Check marked lines independently unless stated otherwise.

Question 1 - Construction, mutability, and field ownership

Define User with username: String, email: String, active: bool, visits: u32.
Write build_user(username: String, email: String) -> User using field init
shorthand. Initialize active to true and visits to 0.

For each marked line, predict whether it compiles and explain why.
Comment out other invalid marked lines to avoid cascading errors.

    let name = String::from("Ada");
    let user = build_user(name, String::from("ada@example.com"));
    println!("{name}");                 // A
    user.visits += 1;                   // B
    let mail = user.email;              // C
    println!("{}", user.username);      // D: keep C
    let whole = user;                   // E: keep C

Keep the build_user parameter types unchanged. Make the smallest changes
that allow visits to increase and name to remain usable after the call.
Explain whether your changes copy the String contents.

Question 2 - Struct update and partial moves

Use a fresh, fully initialized u1 for each case.

Case A:
    let u2 = User { email: String::from("new@example.com"), ..u1 };

Case B:
    let u2 = User {
        username: String::from("Grace"),
        email: String::from("grace@example.com"),
        ..u1
    };

For each case, check these operations independently:
    println!("{}", u1.email);
    println!("{}", u1.active);
    println!("{}", u1.username);
    let original = u1;

Implement both cases. Then change case A so both complete User values remain
usable, copying only the necessary String fields rather than cloning User.
Explain why '..u1 always moves the entire u1' is an inaccurate rule.
*/

// TODO: Define the required types and functions yourself.

fn main() {
    // Question 1: Record predictions, then write and verify your code.
    // TODO: Add your work here.

    // Question 2: Record predictions and explain your implementation.
    // TODO: Add your work here.
}
