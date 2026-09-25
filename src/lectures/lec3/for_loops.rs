fn main() {
    let values = [10, 20, 30];
    let mut sum = 0;
    for value in values {
        sum += value;
    }
    println!("Array sum: {sum}"); // 60

    print!("Exclusive range: ");
    for number in 1..4 {
        print!("{number} ");
    }
    println!();
    print!("Inclusive range: ");
    for number in 1..=4 {
        print!("{number} ");
    }
    println!();
    print!("Reversed range: ");
    for number in (1..4).rev() {
        print!("{number} ");
    }
    println!();

    // iter() borrows the elements; enumerate() adds their indices.
    for (index, value) in values.iter().enumerate() {
        println!("values[{index}] = {value}");
    }

    // These i32 values are copied; changing the binding does not change the array.
    for mut value in values {
        value += 1;
        print!("{value} ");
    }
    println!("Original: {values:?}");

    let mut visits = 0;
    'rows: for row in 0..3 {
        for column in 0..3 {
            if row == 1 && column == 1 {
                break 'rows; // Exit both loops.
            }
            visits += 1;
        }
    }
    println!("Visits before labeled break: {visits}"); // 4
    println!("First even number: {}", first_even()); // 2

    // TODO 2: use enumerate() to find (index, maximum) in [12, 7, 19, 5].
}

fn first_even() -> i32 {
    for number in 1..=5 {
        if number % 2 == 0 {
            return number; // Exit the function, not just the loop.
        }
    }
    -1
}
