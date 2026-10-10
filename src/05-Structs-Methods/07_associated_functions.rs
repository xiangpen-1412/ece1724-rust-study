// ECE1724 — Lecture 5: Associated Functions (Book 5.3)
// Pages: 24 (Lecture 5 slides; title page = 1).
// Topics: functions without self, Self, constructors, Type::function().

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self is short for self: &Self. Read without taking ownership.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // No self parameter: call this associated function on the type.
    // Self means Rectangle inside this impl block.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle::square(3);
    println!("{rectangle:?}"); // Rectangle { width: 3, height: 3 }

    // These are two ways to call the same method.
    let i = Rectangle::area(&rectangle);
    println!("Rectangle::area(&rectangle) = {i}"); // 9
    println!("rectangle.area() = {}", rectangle.area()); // 9
    // rectangle.square(3); // Error: square has no self receiver.
}

// Expand (leave for practice):
// TODO: Add Rectangle::new(width, height); new(3, 5).area() should be 15.
// TODO: Add is_square(&self); compare square(3) with new(3, 5).
