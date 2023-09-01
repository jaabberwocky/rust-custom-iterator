#[derive(Debug, Clone)]
pub struct Page {
    pub page_number: i32,
    pub text: String,
}

impl Page {
    pub fn new(page_number: i32, text: String) -> Page {
        Page { page_number, text }
    }
}
