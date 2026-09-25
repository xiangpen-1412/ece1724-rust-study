// Run: lec3_constants_static

const THRESHOLD: i32 = 10;
const SECONDS_PER_HOUR: i32 = 60 * 60;
static LANGUAGE: &str = "Rust";

fn main() {
    // const: typed compile-time value; static: shared fixed storage.
    println!("Threshold: {THRESHOLD}");
    println!("Seconds per hour: {SECONDS_PER_HOUR}");

    println!("Static language: {LANGUAGE}");

    // Updating this local binding leaves LANGUAGE unchanged.
    let mut language: &str = "Rust";
    println!("Local language before: {language}");
    language = "Go";
    println!("Local language after: {language}");

    println!("Static language afterwards: {LANGUAGE}");

    // THRESHOLD = 5; // Error: const reassignment.
    // LANGUAGE = "Go"; // Error: immutable static.
    // const mut LIMIT: i32 = 100; // Error: const cannot use mut.
    // const LIMIT = 100; // Error: type annotation required.
}
