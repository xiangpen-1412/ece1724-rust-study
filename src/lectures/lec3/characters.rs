// Lecture 3: char, &str, byte literals, and UTF-8 length.
// Run target: lec3_characters
// Single quotes, double quotes, and the b prefix produce different literal types.
// Run the basic example, then compare the lengths of Chinese characters and emoji.

fn main() {
    let letter: char = 'R';
    let text: &str = "Rust";
    let byte: u8 = b'R';
    println!("char：{letter}");
    println!("字符串：{text}");
    println!("字节的数值：{byte}");
    println!("字符串字节数：{}", text.len());

    // TODO 1: Distinguish a char from a string.
    // Create a Chinese char and an emoji char, then try assigning 'ab' to a char.
    // Which literals are valid? Record and comment out deliberate errors.
    // My prediction:
    // Actual result:
    // Explanation:
    // Write your code here:

    // TODO 2: Determine what len() counts.
    // Call len() and chars().count() on "A", "\u{4e2d}", and "\u{1f600}".
    // Predict both sets of results, then print them. Name each kind of length precisely.
    // My prediction:
    // Actual result:
    // Explanation:
    // Write your code here:

    // TODO 3: Does one visible character always correspond to one char?
    // Compare the byte lengths and chars().count() of "\u{e9}" and "e\u{301}".
    // The second string contains e followed by a combining accent; they may look alike.
    // Explain why similar appearance does not determine encoding or scalar value count.
    // My prediction:
    // Actual result:
    // Explanation:
    // Write your code here:
}
