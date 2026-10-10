// ECE1724 — Lecture 5: The Power of Enums
// Pages: 56-61 (Lecture 5 slides; title page = 1).
// Topics: product types, sum types, WebEvent, ChristmasTree, valid states.

// A product type has all its fields together: x AND y.
struct Point {
    x: u32,
    y: u32,
}

// A sum type holds exactly one variant: Click OR PageLoad OR ...
enum WebEvent {
    Click(Point),
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
}

// The slides contrast this with two independent bool fields:
// struct ChristmasTree { alive: bool, growing: bool }
// That permits alive: false, growing: true, an invalid domain state.
// Attach growing only to Alive so the invalid combination cannot be built.
enum ChristmasTree {
    Alive { growing: bool },
    Dead,
}

fn main() {
    // Construct every event shape and extract its associated data.
    let events = [
        WebEvent::PageLoad,
        WebEvent::Click(Point { x: 20, y: 80 }),
        WebEvent::KeyPress('r'),
        WebEvent::Paste(String::from("Rust")),
        WebEvent::PageUnload,
    ];

    for event in events {
        match event {
            WebEvent::Click(Point { x, y }) => println!("Click at ({x}, {y})"),
            WebEvent::PageLoad => println!("Page loaded"),
            WebEvent::PageUnload => println!("Page unloaded"),
            WebEvent::KeyPress(key) => println!("Key pressed: {key}"),
            WebEvent::Paste(text) => println!("Pasted: {text}"),
        }
    }

    // Two variants encode three valid combinations of state and payload.
    let trees = [
        ChristmasTree::Alive { growing: true },
        ChristmasTree::Alive { growing: false },
        ChristmasTree::Dead,
    ];

    for tree in trees {
        match tree {
            ChristmasTree::Alive { growing } => {
                println!("Alive. Growing: {growing}");
            }
            _ => println!("Dead."),
        }
    }

    // The lecture's incomplete match handles Alive but forgets Dead.
    // Removing the Dead arm above causes E0004: non-exhaustive patterns.
    // `ChristmasTree::Dead { growing: true }` cannot be constructed either:
    // Dead has no growing field. The type prevents that invalid state.
    // Exhaustive matching checks coverage, not all business logic.

    // Expand (leave for practice):
    // TODO: Add a Resize { width, height } event and handle its dimensions.
    //       Let the non-exhaustive-match error identify the required update.
    // TODO: Model a different domain where one variant requires extra data;
    //       describe an invalid combination that your enum cannot represent.
}
