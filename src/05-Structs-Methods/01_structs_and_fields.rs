#[derive(Debug)]
struct Book {
    _title: String,
    _author: String,
    _page: u32
}
fn main() {
    // Practice 01: Define a struct, construct instances, and access or change fields.

    let mut book: Book = Book {
        _title: String::from("History 101"),
        _author: String::from("xiangpen"),
        _page: 1000
    };

    // let book1: Book = Book {
    //     _title: String::from("History 101"),
    //     // todo: 'mut' unexpected
    //     mut _author: String::from("xiangpen"),
    //     _page: 1000
    // };

    println!("{}", book._page);

    book._page += 100;

    println!("{}", book._page);

    println!("{:#?}", book);
}
