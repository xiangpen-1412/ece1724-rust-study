/*
05 - Match bindings, exhaustiveness, and result types (Book 6.2)

Question 1 - A pattern binding is not an equality comparison

Predict the complete output and whether the outer y changes, then run:
    let y = 10;
    let x = Some(5);
    let result = match x {
        Some(y) => y + 1,
        None => y,
    };
    println!("{result} {y} {x:?}");

Restore the original snippet before checking each change:
A. Remove the entire None arm.
B. Replace the Some(y) arm's expression with { y + 1; }.
C. Put other => 99 first, followed by Some(y) => y + 1 and None => y.
Classify compilation errors, warnings, or runtime panics and predict output.
Explain named catch-all patterns versus _, and whether an exhaustive match
guarantees correct application logic.

Question 2 - An exhaustive command processor

Define Command with Keep, Add(i32), and Replace(Option<i32>).
Write apply(current: Option<i32>, command: Command) -> Option<i32>:
- Keep preserves current.
- Add(delta) adds only when current contains a value; None stays None.
- Replace(next) uses next, including None.

Use match without unwrap, expect, panic, if let, or a catch-all _ arm.
Assume addition stays within the i32 range.
Predict and check (Some(0), Add(3)), (None, Add(3)),
(Some(7), Replace(None)), and (None, Keep).
Add a unit variant Reset without updating apply and observe the compiler.
Then implement Reset so it produces Some(0).
*/

#[derive(Debug)]
enum Command {
    Keep,
    Add(i32),
    Replace(Option<i32>),
}

fn apply(current: Option<i32>, command: Command) -> Option<i32> {
    match command {
        Command::Keep => { current }
        Command::Add(value) => { Some(value + current.unwrap_or(0)) }
        Command::Replace(value) => { value }
    }
}

fn main() {
    // Question 1: Predict before uncommenting and running this snippet.
    let y = 10;
    let x = Some(5);
    let result = match x {
        Some(y) => y + 1,
        None => y,
    };
    println!("{result} {y} {x:?}");

    // Question 2: Uncomment after implementing apply.
    run_cases();
}

fn run_cases() {
    let cases = [
        (Some(0), Command::Add(3)),
        (None, Command::Add(3)),
        (Some(7), Command::Replace(None)),
        (None, Command::Keep),
    ];
    for (current, command) in cases {
        println!("Input: {current:?}, {command:?}");
        let result = apply(current, command);
        println!("Result: {result:?}");
    }
}
