// Lecture 3, slides 28-32: tuples, destructuring, field access, and unit.
// Run target: lec3_tuples
// Most examples are complete; only the final combined exercise is left for you.

fn rectangle_metrics(width: i32, height: i32) -> (i32, i32) {
    let area = width * height;
    let perimeter = 2 * (width + height);
    // The final expression returns one tuple containing two values.
    (area, perimeter)
}

fn announce() {
    println!("This function returns unit.");
}

fn main() {
    // Each position has its own type; the order and number of positions are fixed.
    let record: (i32, f64, u8) = (120, 2.5, 3);
    println!("Whole tuple: {record:?}");
    println!("Fields: {}, {}, {}", record.0, record.1, record.2);

    // Destructuring creates bindings by position. An underscore ignores a field.
    let (count, price, level) = record;
    let (_, selected_price, _) = record;
    println!("Destructured: {count}, {price}, {level}; selected: {selected_price}");
    // These fields implement Copy, so reading them leaves record usable.
    // Do not generalize this to fields such as String; ownership is a later topic.
    println!("Original tuple remains usable: {record:?}");

    // Tuple field access uses a fixed field number, not a runtime index variable.
    // println!("{}", record.3); // Error: this tuple has no field 3.
    // let index = 1;
    // println!("{}", record[index]); // Error: tuples do not support array indexing.
    // let (a, b) = record; // Error: a two-field pattern cannot match three fields.

    // Mutation changes a value, but cannot change the tuple's shape or field types.
    let mut position: (i32, i32) = (2, 4);
    position.0 = 5;
    println!("Updated position: {position:?}"); // (5, 4)
    // position.1 = 4.5; // Error: field 1 has type i32.
    // position = (1, 2, 3); // Error: a triple is not a pair.

    // Mutability belongs to each new binding independently.
    let (mut row, column) = position;
    row += 1;
    println!("New bindings: ({row}, {column}); original: {position:?}");
    // Expected: new bindings (6, 4), original (5, 4).

    // The trailing comma makes a one-element tuple. Parentheses alone do not.
    let single: (i32,) = (42,);
    println!("Single tuple: {single:?}; grouped number: {}", (42));
    // let wrong: (i32,) = (42); // Error: (42) is an i32, not a tuple.

    // Unit has exactly one value, also written (). It is not an uninhabited type.
    let unit: () = ();
    let returned_unit: () = announce();
    let number = { 7 };
    let no_number = {
        7;
    };
    println!("Unit: {unit:?}, returned unit: {returned_unit:?}");
    println!("Block without semicolon: {number}; with semicolon: {no_number:?}");
    // Expected: 7 and (). The semicolon discards the expression's value.

    // One function result can contain multiple values.
    let (area, perimeter) = rectangle_metrics(3, 4);
    println!("Rectangle: area = {area}, perimeter = {perimeter}"); // 12, 14

    // TODO 1 of 2: combine tuples, destructuring, and shadowing.
    // Start with an immutable point (2_i32, 5_i32). Destructure it into row and column.
    // Shadow point with a new tuple containing (column, row + 1).
    // Print both the old bindings and the new point. Explain why mut is unnecessary.
    // My prediction:
    // My code:
    // Actual result and explanation:
}
