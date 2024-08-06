// Similarly `mod book` and `mod user` will locate the `book.rs`
// and `user.rs` files and insert them here under their respective
// modules
pub mod book;
pub mod user;
use book::Book;
use user::User;
pub struct Library {
    users: Vec<User>,
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            books: Vec::new(),
        }
    }
    pub fn add_user(&mut self, user: User) {
        self.users.push(user)
    }
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }
    pub fn list_checked_out_books(&mut self) {
        for book in &self.books {
            if book.checked_out_by.is_some() {
                let title = &book.title;
                let isbn: &String = &book.isbn;
                let username = book.checked_out_by.clone().unwrap();
                println!("{title} ({isbn}) checked out by {username}")
            }
        }
    }
}
