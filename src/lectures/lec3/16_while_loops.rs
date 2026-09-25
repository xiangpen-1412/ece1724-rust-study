fn main() {
    let mut number = 3;
    while number != 0 {
        println!("Countdown: {number}");
        number -= 1;
    }
    println!("Liftoff!");

    let mut visits = 0;
    while number > 0 {
        visits += 1;
        number -= 1;
    }
    println!("Initially false condition: {visits} visits"); // 0

    let mut current = 0;
    let mut odd_sum = 0;
    while current < 5 {
        current += 1; // Advance before a possible continue.
        if current % 2 == 0 {
            continue;
        }
        odd_sum += current;
    }
    println!("Odd sum from 1 to 5: {odd_sum}"); // 9

    // while and for accept break;, but not a break value.
    // while true { break 42; } // Error: value-bearing break requires loop here.
}
