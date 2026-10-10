fn main() {
    // Practice 11: Shared and mutable array slices.
    test();
}
fn total(values: &[i32]) -> i32 {
    let mut sum = 0;
    for value in values {
        sum += value;
    }
    sum
}

fn demo() {
    let mut arr = [1, 2, 3, 4];
    let p = &mut arr[..3];

    p[0] += 5;
    p[1] = 0;

    println!("{:?}", arr);
}

fn test() {
    let mut values = [10, 20, 30, 40];
    let snapshot = values;

    let part = &mut values[1..3];

    part[0] += 5;
    // println!("{values:?}");
    part[1] *= 2;

    println!("{part:?}");
    println!("{snapshot:?}");
    println!("{values:?}");
}

fn test_2() {
    let mut text = String::from("hello world");
    let p = &text;
    
    println!("{}", text);
    p.len();
}
