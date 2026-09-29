fn main() {
    // Practice 13: Compare UTF-8 bytes, chars, lengths, and slice boundaries.
    let text = "é中!";

    for ch in text.chars() {
        println!("{}", ch);
    }

    for byte in text.bytes() {
        println!("{}", byte);
    }
}
