use library::{book::Book, user::User, Library};
pub mod library {
    use book::Book;
    use user::User;
    pub mod user {
        pub struct User {
            pub username: String,
        }
        impl User {
            pub fn new(username: String) -> Self {
                Self { username }
            }
        }
    }
    pub mod book {
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
    }
    pub struct Library {
        users: Vec<User>,
        books: Vec<Book>,
    }
    impl Default for Library {
        fn default() -> Self {
            Self::new()
        }
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
}
fn main() {
    let mut library = Library::new();
    let user1 = User::new(String::from("john_doe"));
    let mut book1 = Book::new(
        String::from("The Rust Programming Language"),
        String::from("123"),
    );
    book1.check_out(user1.username.clone());
    library.add_user(user1);
    library.add_book(book1);
    library.list_checked_out_books();
}
