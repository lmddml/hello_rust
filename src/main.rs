// This declaration will look for a file named `library.rs` and will
// insert its contents inside a module named `library` under this scope
mod library;
use library::{book::Book, user::User, Library};

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
