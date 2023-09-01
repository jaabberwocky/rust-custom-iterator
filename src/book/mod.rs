#![allow(dead_code, unused_variables)]

use page::Page;

#[derive(Debug, Clone)]
pub struct Book {
    name: String,
    year_published: i32,
    contents: Vec<Page>,
    last_page: i32,
}

pub struct BookIter<'a> {
    book: &'a Book,
    current_page: usize,
}

impl<'a> Iterator for BookIter<'a> {
    type Item = &'a Page;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page < self.book.contents.len() {
            let result = &self.book.contents[self.current_page];
            self.current_page += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl Book {
    pub fn iter(&self) -> BookIter {
        BookIter {
            book: self,
            current_page: 0,
        }
    }

    pub fn new(name: String, year_published: i32) -> Book {
        Book {
            name,
            year_published,
            contents: Vec::new(),
            last_page: 0i32,
        }
    }

    pub fn add_page(&mut self, contents: String) {
        self.last_page += 1;
        let p = Page::new(self.last_page, contents);
        self.contents.push(p);
    }

    pub fn display_pages(&self) {
        for page in &self.contents {
            println!("Page {} - {}", page.page_number, page.text);
        }
    }
}

pub mod page;
