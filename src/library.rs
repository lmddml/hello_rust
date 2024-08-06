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
            if let Some(username) = &book.checked_out_by {
                let Book { title, isbn, .. } = book;
                println!("{} ({}) checked out by {}", title, isbn, username);
            }
        }
    }

    pub fn count_checked_out_books_by_user(&mut self, username: &String) -> i32 {
        let mut i = 0;
        for book in &self.books {
            if let Some(checked_out_by) = &book.checked_out_by {
                if username == checked_out_by {
                    i += 1;
                }
            }
        }
        i
    }
}
