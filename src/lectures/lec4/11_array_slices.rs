fn main() {
    let values = [10, 20, 30, 40, 50];
    let middle: &[i32] = &values[1..4];
    println!("Middle: {middle:?}; length: {}", middle.len()); // 3 elements.
    println!("middle[0]: {}; values[1]: {}", middle[0], values[1]);
    println!("Middle sum: {}; full sum: {}", sum(middle), sum(&values));

    // A slice borrows; assigning this Copy array duplicates its elements.
    let mut copied = values;
    copied[0] = 99;
    println!("Original array: {values:?}; copied array: {copied:?}");

    let mut scores = [1, 2, 3, 4, 5];
    let selected: &mut [i32] = &mut scores[1..4];
    add_one(selected);
    // The mutable borrow ends after its last use.
    println!("Original after update: {scores:?}"); // [1, 3, 4, 5, 5]

    let empty: &[i32] = &values[2..2];
    println!("Empty slice: {empty:?}; sum: {}", sum(empty));
    // println!("{}", middle[middle.len()]); // Panics: index equals length.
}

fn sum(items: &[i32]) -> i32 {
    let mut total = 0;
    for &value in items {
        total += value;
    }
    total
}

fn add_one(items: &mut [i32]) {
    for value in items {
        *value += 1;
    }
}
