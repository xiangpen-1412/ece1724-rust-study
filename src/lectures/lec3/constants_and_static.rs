// Lecture 3, slides 5-7: const, static, and switching to a local let mut binding.
// Run target: lec3_constants_static

const THRESHOLD: i32 = 10;
const SECONDS_PER_HOUR: i32 = 60 * 60;
static LANGUAGE: &str = "Rust";

fn main() {
    // A const requires a type annotation; its initializer can be a compile-time expression.
    println!("Threshold: {THRESHOLD}");
    println!("Seconds per hour: {SECONDS_PER_HOUR}");

    // An immutable static can be read directly and has fixed storage for the program's lifetime.
    // A const defines a value; its uses are not guaranteed to share the same address.
    println!("Static language: {LANGUAGE}");

    // Slide 7: use a local mutable variable when this function needs to change the value.
    // language and LANGUAGE above are different names with separate bindings.
    let mut language: &str = "Rust";
    println!("Local language before: {language}");
    language = "Go";
    println!("Local language after: {language}");

    // Reassignment changes only the local language; LANGUAGE remains "Rust".
    println!("Static language afterwards: {LANGUAGE}");

    // Error experiments: uncomment one line at a time.
    // THRESHOLD = 5; // A const cannot be reassigned.
    // LANGUAGE = "Go"; // An immutable static cannot be reassigned.
    // const mut LIMIT: i32 = 100; // const cannot be combined with mut.
    // const LIMIT = 100; // A const declaration requires a type annotation.

    // An unsafe block does not make the LANGUAGE static assignable.
    // static mut is a separate topic and is not needed in this example.
}
