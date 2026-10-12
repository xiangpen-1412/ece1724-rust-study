struct Book {
    title: String,
    pages: u32,
}

fn build_book(title: String, pages: u32) -> Book {
    Book {
        title,
        pages,
    }
}
fn main() {
    // Practice 03: Trace moved, copied, and untouched fields in struct updates.
    let book = build_book("history".to_string(), 200);

    let mut book2 = Book {
        pages: 100,
        ..book
    };

    book2.title = String::from("book2");

    // note: Value used after being moved
    // println!("{}", book.title);
}
