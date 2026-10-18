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
    let mut counter = 7;

    loop {
        if counter > 20 {
            break;
        }
        counter += 7;
    }

    println!("{}", counter);

    // let flag = true;
    //
    // let result = loop {
    //     if flag {
    //         break 10;
    //     } else {
    //         break "hello";
    //     }
    // };

    'inner: loop {
        break
    }

    'outer: loop {
         for n in 1..3 {
             if n == 2 {break 'outer;}
         }
    };

    let mut n: i32 = 9;

    loop {
        n = 10;
        println!("{}", n);
        break;
    }

    println!("{}", n);
}
