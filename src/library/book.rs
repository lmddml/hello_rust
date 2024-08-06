pub struct Book {
    pub title: String,
    pub isbn: String,
    pub checked_out_by: Option<String>,
}
impl Book {
    pub fn new(title: String, isbn: String) -> Self {
        Self {
            title,
            isbn,
            checked_out_by: None,
        }
    }
    pub fn check_out(&mut self, user: String) {
        self.checked_out_by = Some(user)
    }
}
