// ECE1724 — Lecture 5: Object-Oriented Characteristics (Book 18.1)
// Pages: 62-67 (Lecture 5 slides; title page = 1).
// Topics: data and behavior, encapsulation, inheritance, polymorphism.
// The slides discuss concepts; this small example illustrates those concepts.
// It is not a verbatim code example from the slides.

mod audio {
    // The type is public, but its representation stays private to this module.
    pub struct Volume {
        level: u8,
    }

    // Data lives in the struct; associated behavior lives in its impl.
    impl Volume {
        pub fn new() -> Self {
            Self { level: 0 }
        }

        // A shared borrow reads the value without changing or consuming it.
        pub fn level(&self) -> u8 {
            self.level
        }
    }
}

fn main() {
    let volume = audio::Volume::new();
    println!("Initial volume: {}", volume.level());

    // Encapsulation: callers use the public interface to access private data.
    // println!("{}", volume.level); // E0616: private field outside audio.
    // Privacy follows module boundaries, not a special methods-only rule.
    // Code in audio and its child modules can access the private field.

    // Is Rust object-oriented? It depends on the definition:
    // - Data + behavior: structs/enums with impl blocks support this.
    // - Encapsulation: private fields and public methods support this.
    // - Class inheritance: Rust has no inheritance of struct fields.
    //   Composition stores another type as a field to reuse its functionality.

    // Later-course preview; no trait implementation is needed in this example.
    // Traits describe shared behavior and may provide default methods.
    // Generics with trait bounds (T: Trait) support static dispatch.
    // Trait objects (&dyn Trait) support dynamic dispatch at runtime.
    // Neither mechanism gives structs inherited fields.

    // Expand (leave for practice):
    // TODO: Add set_level(&mut self, requested: u8) -> bool; accept 0..=10,
    //       reject larger values, and preserve the current level on rejection.
    // TODO: Create a Player containing a Volume and expose a forwarding getter.
    //       Explain why containment does not make Player a subtype of Volume.
}
