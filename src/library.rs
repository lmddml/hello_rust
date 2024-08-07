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
    pub fn list_all_users(&self) {
        for user in &self.users {
            println!("{}", user.username);
        }
    }
    fn count_checked_out_books_by_user(&mut self, username: &String) -> usize {
        self.books
            .iter()
            .filter(|book| book.checked_out_by == Some(username.to_string()))
            .count()
    }
    fn find_and_check_out_book(&mut self, username: &str, title: &str) {
        let book_to_check_out = self.books.iter_mut().find(|book| book.title == title);

        match book_to_check_out {
            Some(book) => {
                book.check_out(username.to_string());
            }
            None => {
                println!("Book not found");
            }
        }
    }
    pub fn check_out_book(&mut self, username: &str, title: &str) {
        let n = self.count_checked_out_books_by_user(&username.to_string());
        if n == 2 {
            println!("Too many books!");
            return;
        }

        self.find_and_check_out_book(username, title);
    }
}
