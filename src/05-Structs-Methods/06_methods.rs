// ECE1724 — Lecture 5: Methods (Book 5.3)
// Pages: 20-23 (Lecture 5 slides; title page = 1).
// Topics: impl, shared receivers, method calls, multiple impl blocks.

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

    // Parentheses distinguish this getter from the width field.
    fn width(&self) -> u32 {
        self.width
    }
}

// Multiple impl blocks add methods to the same type.
impl Rectangle {
    // Borrow both values; compare dimensions strictly without rotation.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Add your expansion methods in another impl Rectangle block here.

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    // Dot syntax automatically borrows the receiver for area(&self).
    println!("area: {}", rect.area()); // 200
    println!("still usable: {rect:?}");
    println!("width field: {}", rect.width); // 10
    println!("width method: {}", rect.width()); // 10

    // A reference can call the same method without a -> operator.
    let borrowed = &rect;
    println!("area through &Rectangle: {}", borrowed.area()); // 200

    let smaller = Rectangle {
        width: 5,
        height: 10,
    };
    let too_wide = Rectangle {
        width: 15,
        height: 5,
    };
    let same_size = Rectangle {
        width: 10,
        height: 20,
    };

    println!("holds smaller: {}", rect.can_hold(&smaller)); // true
    println!("holds too_wide: {}", rect.can_hold(&too_wide)); // false
    println!("holds same_size: {}", rect.can_hold(&same_size)); // false
    println!("smaller is still usable: {smaller:?}");
    // rect.can_hold(smaller); // Error: ordinary arguments still need an explicit &.

    // Expand (leave for practice):
    // TODO 1: Add resize(&mut self, width, height) and into_dimensions(self).
    // Make rect mutable, resize to 30x40, check area 1200, then return (30, 40).
    // Explain why a later use of rect fails after the consuming method.
    // Also test a shared reference used after resize: why do the borrows conflict?
    // TODO 2: Add perimeter(&self) and rotate(&mut self).
    // A fresh 3x8 rectangle should have perimeter 22; after rotation it is 8x3.
    // Verify that area and perimeter stay unchanged after rotation.
}
