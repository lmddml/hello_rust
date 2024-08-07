// This declaration will look for a file named `library.rs` and will
// insert its contents inside a module named `library` under this scope
mod library;
use library::{book::Book, user::User, Library};

fn main() {
    let mut library = Library::new();
    let user1 = User::new(String::from("john_doe"));
    let book1 = Book::new(
        String::from("The Rust Programming Language"),
        String::from("123"),
    );
    let book2 = Book::new(String::from("Software Engineering"), String::from("456"));
    let book3 = Book::new(String::from("Computer Science"), String::from("789"));
    library.add_user(user1);
    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);

    library.check_out_book("john_doe", "The Rust Programming Language");
    library.check_out_book("john_doe", "Software Engineering");
    library.check_out_book("john_doe", "Computer Science");

    library.list_all_users();
    library.list_checked_out_books();
}
