// Lecture 3: char, &str, byte literals, and UTF-8 length.
// Run target: lec3_characters
// Distinguish bytes, Unicode scalar values, and visually perceived characters.
// Unicode test data appears in code; all explanations are in English.

fn main() {
    let letter: char = 'R';
    let text: &str = "Rust";
    let byte: u8 = b'R';
    println!("char：{letter}");
    println!("字符串：{text}");
    println!("字节的数值：{byte}");
    println!("字符串字节数：{}", text.len());

    // Single quotes form a char literal; double quotes form a string literal.
    // A char holds exactly one Unicode scalar value, including non-ASCII values.
    let chinese: char = '中';
    let emoji: char = '😀';
    println!("Valid char values: {chinese}, {emoji}");
    // let invalid: char = 'ab'; // Error: a char literal contains multiple characters.
    // let wrong_type: char = "R"; // Error: a string literal is not a char.

    // Your original string and iterator declarations are preserved and used.
    let text = "中";
    let chars = text.chars();
    let scalar_count = chars.count();
    println!(
        "Your text: {} bytes, {scalar_count} scalar value",
        text.len()
    );
    // Expected: 3 bytes and 1 scalar value.
    // count consumes the iterator; create a new iterator to count again.
    // let second_count = chars.count(); // Error: chars was moved by count().
    println!("Count with a fresh iterator: {}", text.chars().count());

    // str::len counts UTF-8 bytes. chars().count counts Unicode scalar values.
    // Expected byte/scalar counts: ASCII 1/1, Chinese 3/1, emoji 4/1.
    show_lengths("ASCII", "A");
    show_lengths("Chinese", "中");
    show_lengths("Emoji", "😀");

    // A stored Rust char always occupies 4 bytes, independent of UTF-8 length.
    // len_utf8 reports the bytes needed to encode a particular char in UTF-8.
    println!(
        "Storage occupied by char: {} bytes",
        std::mem::size_of::<char>()
    );
    println!(
        "UTF-8 encoding lengths: {}, {}, {}",
        letter.len_utf8(),
        chinese.len_utf8(),
        emoji.len_utf8()
    ); // Expected: 1, 3, 4.

    // A byte literal has type u8. Here the ASCII code of R is 82.
    println!("ASCII byte value: {byte}");
    // let invalid_byte = b'\u{4e2d}'; // Error: byte literals cannot use Unicode escapes.
    println!("UTF-8 bytes of your text: {:?}", text.as_bytes());
    // Expected: [228, 184, 173]. Those three bytes encode one scalar value.

    // These strings can look alike but contain different scalar sequences.
    // Rust does not normalize Unicode automatically when comparing strings.
    let precomposed = "é";
    let combining = "e\u{301}";
    show_lengths("Precomposed accent", precomposed); // Expected: 2 bytes, 1 scalar.
    show_lengths("Combining accent", combining); // Expected: 3 bytes, 2 scalars.
    println!("Equal strings: {}", precomposed == combining); // Expected: false.

    // A visible character may contain multiple scalar values, so chars().count()
    // is not a general way to count user-perceived characters (grapheme clusters).
    // Strings also cannot be indexed with an integer to obtain a char.
    // let first = text[0]; // Error: str cannot be indexed by an integer.
    println!("First scalar value: {:?}", text.chars().next());
    // next returns Some(char) here, or None if the string is empty.
}

fn show_lengths(label: &str, text: &str) {
    println!(
        "{label}: {} bytes, {} scalar values",
        text.len(),
        text.chars().count()
    );
}
