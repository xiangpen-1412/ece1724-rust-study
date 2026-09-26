fn main() {
    // Copy leaves the source usable.
    let number: i32 = 10;
    let mut copied_number = number;
    copied_number += 5;
    println!("Integer: {number}; copy after change: {copied_number}");

    // &str is Copy; copying it does not duplicate the text.
    let label: &str = "Rust";
    let copied_label = label;
    println!("Label: {label}; copied label: {copied_label}");

    // A tuple is Copy when all its fields are Copy.
    let metrics = (12_i32, true);
    let copied_metrics = metrics;
    println!("Tuple: {metrics:?}; copied tuple: {copied_metrics:?}");

    let record = (String::from("Rust"), 3_i32);
    let moved_record = record;
    println!("Moved tuple: {moved_record:?}");
    // println!("{record:?}"); // E0382: the tuple contains a non-Copy String.

    // An array is Copy when its element type is Copy.
    let numbers = [1, 2, 3];
    let mut copied_numbers = numbers;
    copied_numbers[0] = 9;
    println!("Array: {numbers:?}; changed copy: {copied_numbers:?}");

    let names = [String::from("Ada"), String::from("Lin")];
    let moved_names = names;
    println!("Moved String array: {moved_names:?}");
    // println!("{names:?}"); // E0382: String elements make this array non-Copy.

    // Partial move: the remaining Copy field is still usable.
    let item = (String::from("book"), 2_i32);
    let title = item.0;
    println!("Moved field: {title}; remaining field: {}", item.1);
    // println!("{item:?}"); // E0382: the whole tuple is partly moved.
}
