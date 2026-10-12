/*
03 - Methods and associated functions (Book 5.3)

Question 1 - Receivers and the last use of a borrow

Rectangle and routine setup are provided, without Copy or Clone.
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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// TODO: Write your own impl blocks and methods here.
// Start with area, resize, and into_dimensions.
// Add new, square, and can_hold after the associated-functions lesson.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

impl Rectangle {
    // take the ownership of self
    fn into_dimensions(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Self::new(size, size)
    }

    fn can_hold(&self,  other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut r = Rectangle { width: 3, height: 4 };
    println!("Initial rectangle: {r:?}");

    // Uncomment each step when you have implemented the required method.
    println!("Initial area: {}", r.area());
    r.resize(6, 8);
    println!("Resized rectangle: {r:?}");
    println!("Resized area: {}", r.area());
    let dimensions = r.into_dimensions();
    println!("Consumed dimensions: {dimensions:?}");

    // note: Value used after being moved
    // println!("{:?}", r);

    let outer = Rectangle::new(6, 8);
    let inner = Rectangle::square(3);
    println!("Can hold 3x3: {}", outer.can_hold(&inner));
    println!("Can hold 6x7: {}", outer.can_hold(&Rectangle::new(6, 7)));
    println!("Can hold 7x5: {}", outer.can_hold(&Rectangle::new(7, 5)));
    println!("Still available: {outer:?}, {inner:?}");

    println!("{}", Rectangle::can_hold(&Rectangle { width: 3, height: 4 }, &Rectangle { width: 5, height: 6 }));
}
