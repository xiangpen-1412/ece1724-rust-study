// ECE1724 — Lecture 5: Concise Control Flow (Book 6.3)
// Pages: 55 (Lecture 5 slides; title page = 1).
// Topics: if let, matching one pattern, comparison with match.

fn main() {
    // The lecture uses Option<u8>: a maximum may or may not be configured.
    let config_max = Some(3u8);

    println!("Using match:");
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (), // Ignore the remaining case (None).
    }

    // Same behavior, with less syntax when only one pattern matters.
    println!("Using if let:");
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // `max` exists only inside its matching arm or if-let body.
    // `Some(max)` is a pattern, not an equality comparison.
    // Option<u8> is Copy, so the previous matches leave config_max usable.
    println!("The original option is still {config_max:?}");

    // A failed pattern match skips the body when there is no else.
    let missing_max: Option<u8> = None;
    if let Some(max) = missing_max {
        println!("This message is skipped: {max}");
    }
    println!("Continued after the skipped body.");

    // Small classroom follow-up: else handles every nonmatching value.
    if let Some(max) = missing_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("No maximum is configured.");
    }

    // With an exhaustive match, each possible case must be covered.
    // if let intentionally handles one pattern; it needs no exhaustive list.
    // It is useful when the ignored cases need no individual handling.

    // Expand (leave for practice):
    // TODO: Write a helper that prints Some values and a fallback for None.
    //       Try Some(0), Some(8), and None; zero must remain a valid value.
    // TODO: Rewrite that helper with match and confirm identical output.
}
