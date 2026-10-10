/*
03 - Methods and associated functions (Book 5.3)

Question 1 - Receivers and the last use of a borrow

Define Rectangle with u32 width and height, without Copy or Clone.
Implement area (read-only), resize(width, height) (mutates dimensions),
and into_dimensions (consumes Rectangle and returns (u32, u32)).
Choose the appropriate receiver for each method.

    let mut r = Rectangle { width: 3, height: 4 };
    let view = &r;
    r.resize(6, 8);                            // A
    println!("{}", view.area());              // B
    let dimensions = r.into_dimensions();
    println!("{}", r.area());                 // C

Identify both sites involved in the borrow conflict and the separate error.
Without cloning or constructing another Rectangle, change the order of the
last use of view and the consuming call. Observe the original area, then the
new area, and finally obtain dimensions. Explain your repair.

Question 2 - Associated functions and automatic borrowing

Add new(width, height) -> Self and square(size) -> Self.
In a second impl block, add can_hold, which reads two Rectangles.
No rotation is allowed; both outer dimensions must be strictly larger.
Predict each independent call before running it:

    let outer = Rectangle::new(6, 8);
    let inner = Rectangle::square(3);
    outer.can_hold(inner);
    outer.can_hold(&inner);
    Rectangle::can_hold(&outer, &inner);
    outer.square(4);

For valid calls, can outer and inner still be used? Where does automatic
borrowing apply? Check 6x8 containing 3x3, 6x7, and 7x5.
*/

// TODO: Define the required types and functions yourself.

fn main() {
    // Question 1: Record predictions, then write and verify your code.
    // TODO: Add your work here.

    // Question 2: Record predictions and explain your implementation.
    // TODO: Add your work here.
}
