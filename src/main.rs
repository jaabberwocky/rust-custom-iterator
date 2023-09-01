use custom_iterator::book::Book;

fn main() {
    let mut b: Book = Book::new("The Name of the Wind".to_string(), 2007);

    b.add_page("First Page".to_string());
    b.add_page("Second Page".to_string());
    b.add_page("Third Page".to_string());

    b.display_pages();
    println!("{:?}", &b);

    for page in b.iter() {
        println!("Page {} - {}", page.page_number, page.text);
    }
    println!("{:?}", &b);
}
