// Characters and UTF-8.
// Run: lec3_characters

fn main() {
    let letter: char = 'R';
    let text: &str = "Rust";
    let byte: u8 = b'R';
    println!("char：{letter}");
    println!("字符串：{text}");
    println!("字节的数值：{byte}");
    println!("字符串字节数：{}", text.len());

    // char holds one Unicode scalar value.
    let chinese: char = '中';
    let emoji: char = '😀';
    println!("Valid char values: {chinese}, {emoji}");
    // let invalid: char = 'ab'; // Error: multiple characters.
    // let wrong_type: char = "R"; // Error: &str, not char.

    let text = "中";
    let chars = text.chars();
    let scalar_count = chars.count();
    println!(
        "Your text: {} bytes, {scalar_count} scalar value",
        text.len()
    );
    // count consumes the iterator.
    // let second_count = chars.count(); // Error: moved iterator.
    println!("Count with a fresh iterator: {}", text.chars().count());

    // len: bytes; chars().count(): scalar values.
    show_lengths("ASCII", "A");
    show_lengths("Chinese", "中");
    show_lengths("Emoji", "😀");

    // char storage: 4 bytes; UTF-8 encoding: 1-4 bytes.
    println!(
        "Storage occupied by char: {} bytes",
        std::mem::size_of::<char>()
    );
    println!(
        "UTF-8 encoding lengths: {}, {}, {}",
        letter.len_utf8(),
        chinese.len_utf8(),
        emoji.len_utf8()
    ); // 1, 3, 4.

    println!("ASCII byte value: {byte}");
    // let invalid_byte = b'\u{4e2d}'; // Error: Unicode byte escape.
    println!("UTF-8 bytes of your text: {:?}", text.as_bytes());

    // String comparison does not normalize Unicode.
    let precomposed = "é";
    let combining = "e\u{301}";
    show_lengths("Precomposed accent", precomposed); // 2 bytes, 1 scalar.
    show_lengths("Combining accent", combining); // 3 bytes, 2 scalars.
    println!("Equal strings: {}", precomposed == combining); // false.

    // Scalar counts can differ from visible-character counts.
    // let first = text[0]; // Error: integer string index.
    println!("First scalar value: {:?}", text.chars().next());
    // next: Some(char), or None when empty.
}

fn show_lengths(label: &str, text: &str) {
    println!(
        "{label}: {} bytes, {} scalar values",
        text.len(),
        text.chars().count()
    );
}
