// Lecture 3, slide 4: errors involving immutable variables and how to fix them.
// Covers the previous slide's mutable/immutable example and the score exercise.
// Run target: lec3_variables

fn main() {
    // let is immutable by default, regardless of whether the variable is named immutable.
    let immutable = 1;
    println!("Immutable value: {immutable}");

    // mut allows reassignment after a variable has been initialized.
    let mut mutable = 1;
    println!("Before mutation: {mutable}");
    mutable += 1;
    println!("After mutation: {mutable}");

    // For these integers, += and the assignment below perform the same update.
    let mut score = 10;
    score = score + 5;
    println!("Score: {score}");

    // Error experiment: uncomment one line at a time and inspect the compiler diagnostic.
    // immutable += 1; // E0384: cannot reassign an immutable variable.

    // You can also remove mut from the score declaration above to see the reassignment error.
    // If compilation fails, none of this program's print statements execute.
}
