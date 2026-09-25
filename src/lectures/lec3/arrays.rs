// Lecture 3, slides 33-35: fixed-size arrays, indexing, and bounds.
// Run target: lec3_arrays
// Normal runs finish successfully. Optional CLI arguments demonstrate indexing:
// cargo run --bin lec3_arrays -- 2
// cargo run --bin lec3_arrays -- 4
// cargo run --bin lec3_arrays -- 4 --panic
// The final command deliberately panics; the others use checked access.

fn main() {
    // An array type includes both the element type and the length.
    let values: [i32; 4] = [10, 20, 30, 40];
    println!("Array: {values:?}; length: {}", values.len());
    println!("First: {}; last: {}", values[0], values[3]);
    // let wrong: [i32; 3] = values; // Error: [i32; 4] and [i32; 3] differ.
    // let mixed = [10, 2.5]; // Error: array elements need one common type.

    // A comma lists elements; a semicolon repeats one value a fixed number of times.
    let listed = [3, 5];
    let repeated = [3; 5];
    println!("Listed: {listed:?}; repeated: {repeated:?}");
    // Expected: [3, 5] and [3, 3, 3, 3, 3].
    let empty: [i32; 0] = [];
    println!("Empty array length: {}", empty.len());

    // mut allows replacing elements, not growing the array or changing its type.
    let mut scores = [60, 70, 80];
    scores[1] = 95;
    println!("Updated scores: {scores:?}"); // [60, 95, 80]
    // scores = [1, 2]; // Error: a two-element array has a different type.
    // scores.push(90); // Error: arrays have no push method; Vec is a later topic.

    // Indexing an array with one integer uses usize. The index may vary at runtime.
    let index: usize = 2;
    println!("values[{index}] = {}", values[index]); // 30
    // let signed_index: i32 = 2;
    // println!("{}", values[signed_index]); // Error: an i32 is not a valid index type.

    // get returns Some(&value) in bounds, or None out of bounds, without panicking.
    // Option and references are later topics; observe these results for now.
    println!("Checked index 3: {:?}", values.get(3)); // Some(40)
    println!("Checked index 4: {:?}", values.get(4)); // None
    // Valid indices satisfy index < values.len(); equality is already out of bounds.
    // println!("{}", values[4]); // Statically known invalid indexing is normally rejected.
    // Dynamic invalid indexing panics, including in a release build.

    // Nested arrays store fixed-size rows. This is a small grid, not assignment code.
    let mut grid: [[i32; 3]; 2] = [[0; 3]; 2];
    grid[1][2] = 9;
    println!("Grid: {grid:?}"); // [[0, 0, 0], [0, 0, 9]]
    println!("Rows: {}; columns per row: {}", grid.len(), grid[0].len());
    // grid[row][column]: the outer index selects a row, then the inner index a column.
    // The type means two rows, each containing three i32 values.

    // Optional complete runtime experiment; no input is required for a normal run.
    // Read index and mode from command-line arguments supplied after Cargo's --.
    if let Some(argument) = std::env::args().nth(1) {
        let runtime_index: usize = argument.parse().expect("Expected a non-negative index");
        let panic_mode = std::env::args().nth(2).as_deref() == Some("--panic");
        if panic_mode {
            println!("Direct indexing: {}", values[runtime_index]);
        } else {
            println!("Checked indexing: {:?}", values.get(runtime_index));
        }
    }

    // TODO 2 of 2: combine nested arrays, tuple coordinates, and bounds checking.
    // Create a 2-by-3 zero-filled i32 grid and a position: (usize, usize).
    // Destructure the position and check both bounds with && before writing 7.
    // Print the grid for a valid position; print a message for an invalid one.
    // Try positions (1, 2) and (2, 0). Do not rely on a panic to reject the latter.
    // My prediction:
    // My code:
    // Actual result and explanation:
}
