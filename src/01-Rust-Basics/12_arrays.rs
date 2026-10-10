// Arrays and bounds.
// Run: lec3_arrays
// cargo run --bin lec3_arrays -- 2
// cargo run --bin lec3_arrays -- 4
// cargo run --bin lec3_arrays -- 4 --panic
// --panic uses direct indexing; index 4 panics.

fn main() {
    // Type includes length.
    let values: [i32; 4] = [10, 20, 30, 40];
    println!("Array: {values:?}; length: {}", values.len());
    println!("First: {}; last: {}", values[0], values[3]);
    // let wrong: [i32; 3] = values; // Error: length mismatch.
    // let mixed = [10, 2.5]; // Error: mixed element types.

    // List versus repetition.
    let listed = [3, 5];
    let repeated = [3; 5];
    println!("Listed: {listed:?}; repeated: {repeated:?}");
    let empty: [i32; 0] = [];
    println!("Empty array length: {}", empty.len());

    // mut cannot resize an array.
    let mut scores = [60, 70, 80];
    scores[1] = 95;
    println!("Updated scores: {scores:?}"); // [60, 95, 80]
    // scores = [1, 2]; // Error: length mismatch.
    // scores.push(90); // Error: no push method.

    // Integer indices use usize.
    let index: usize = 2;
    println!("values[{index}] = {}", values[index]); // 30
    // let signed_index: i32 = 2;
    // println!("{}", values[signed_index]); // Error: index must be usize.

    // get: Some(&value) or None.
    println!("Checked index 3: {:?}", values.get(3)); // Some(40)
    println!("Checked index 4: {:?}", values.get(4)); // None
    // println!("{}", values[4]); // Rejected: known out-of-bounds index.
    // Invalid dynamic indexing panics, even in release.

    // grid[row][column]: two rows, three columns.
    let mut grid: [[i32; 3]; 2] = [[0; 3]; 2];
    grid[1][2] = 9;
    println!("Grid: {grid:?}"); // [[0, 0, 0], [0, 0, 9]]
    println!("Rows: {}; columns per row: {}", grid.len(), grid[0].len());

    // Optional index and --panic CLI arguments.
    if let Some(argument) = std::env::args().nth(1) {
        let runtime_index: usize = argument.parse().expect("Expected a non-negative index");
        let panic_mode = std::env::args().nth(2).as_deref() == Some("--panic");
        if panic_mode {
            println!("Direct indexing: {}", values[runtime_index]);
        } else {
            println!("Checked indexing: {:?}", values.get(runtime_index));
        }
    }

    // TODO: Build a 2x3 grid; destructure a (usize, usize) position and check both bounds.
    // Write 7 if valid; otherwise print a message. Try (1, 2) and (2, 0).
}
