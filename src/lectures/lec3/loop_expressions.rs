// loop: break can supply the loop's result.
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("counter = {counter}, result = {result}"); // 10, 20

    let unit = loop {
        if counter == 0 {
            break;
        }
        counter -= 1;
    };
    println!("counter = {counter}, loop value = {unit:?}"); // 0, ()

    // Break values targeting the same loop must have compatible types.
    // let invalid = loop { if true { break 1; } else { break "one"; } };

    // TODO 1: find the first multiple of 7 greater than 20 using loop and break value.
}
