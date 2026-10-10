// Tuples and unit.
// Run: lec3_tuples

fn rectangle_metrics(width: i32, height: i32) -> (i32, i32) {
    let area = width * height;
    let perimeter = 2 * (width + height);
    (area, perimeter)
}

fn announce() {
    println!("This function returns unit.");
}

fn main() {
    // Fixed field types and order.
    let record = (120, 2.5, 3);
    println!("{:?}", record);
    println!("{:#?}", record);
    let one = record.0;
    let two = record.1;
    let three = record.2;
    println!("{}, {}, {}", one, two, three);

    // Destructure by position; _ ignores a field.
    let (count, price, level) = record;
    let (_, selected_price, _) = record;
    println!("Destructured: {count}, {price}, {level}; selected: {selected_price}");
    // Copy fields stay usable; String fields can move.
    println!("Original tuple remains usable: {record:?}");

    // println!("{}", record.3); // Error: missing field.
    // let index = 1;
    // println!("{}", record[index]); // Error: no tuple indexing.
    // let (a, b) = record; // Error: field count mismatch.

    // mut changes values, not types or shape.
    let mut position: (i32, i32) = (2, 4);
    position.0 = 5;
    println!("Updated position: {position:?}"); // (5, 4)
    // position.1 = 4.5; // Error: expected i32.
    // position = (1, 2, 3); // Error: tuple shape mismatch.

    let (mut row, _col) = position;
    row = row + 4;
    println!("Updated row: {row}");
    println!("Original position: {position:?}");
    // row: 9; original position: (5, 4).

    // One-element tuples need a comma.
    let single: (i32,) = (42,);
    println!("Single tuple: {single:?}; grouped number: {}", (42));
    // let wrong: (i32,) = (42); // Error: missing comma.

    // Unit has one value: ().
    let unit: () = ();
    let returned_unit: () = announce();
    let number = { 7 };

    // The semicolon makes this block return ().
    let no_number = {
        7;
    };
    println!("Unit: {unit:?}, returned unit: {returned_unit:?}");
    println!("Block without semicolon: {number}; with semicolon: {no_number:?}");

    let width = 3;
    let height = 4;
    let (area, perimeter) = rectangle_metrics(width, height);
    println!("Rectangle: area = {area}, perimeter = {perimeter}"); // 12, 14

    println!("{}", width);

    // TODO: Destructure (2_i32, 5_i32); shadow point with (column, row + 1).
    // Print old bindings and new point; explain why mut is unnecessary.

    let hello = String::from("Hello, ");
    let point = (hello, 0);
    println!("{:?}", point);

    let point_a = point;
    println!("{:?}", point_a);
    // let (row, col) = point;
    // let point = (col, row);
    // println!("{:?}", point);
}
