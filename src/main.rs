mod utils;
mod actions;
use std::io;
use utils::remove_trailing_whitespace;
use actions::{print_books, add_book, remove_book};

fn main() {
    let mut username = String::new();
    let mut collection = vec![[
        "PEP 20 - The Zen of Python".to_string(), // title
        "Tim Peters".to_string(), // author
        "Owned".to_string(), // type: Owned | Rented | Borrowed
        "19-Aug-2004".to_string(), // owned date
        String::new(), // borrowed start date
        String::new(), // borrowed end date
        String::new(), // rent start date
        String::new(), // rent end date
        String::new(), // rent price
    ]];
    let mut n_books: u8 = collection.len() as u8;

    println!("Enter name: ");
    io::stdin()
        .read_line(&mut username)
        .expect("failed to read line"); // pointers are normally immutable
    remove_trailing_whitespace(&mut username); // remove any trailing whitespaces

    println!("Welcome to your library {username}. You currently have {n_books} in your collection.");
    loop {
        println!("\nWhat would you like to do?\n\
        1. Add new books\n\
        2. Check added books\n\
        3. Remove a book\n\
        4. Exit\n");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to read line");
        remove_trailing_whitespace(&mut choice);

        match choice.parse::<u8>() {
            Ok(ok) => {
                match ok {
                    1 => add_book(&mut collection, &mut n_books),
                    2 => {
                        if n_books == 0 {println!("You currently have no books to check")}
                        print_books(&collection)
                    },
                    3 => {
                        if n_books == 0 {println!("You currently have no books to delete")}
                        remove_book(&mut collection, &mut n_books)
                    }
                    4 => {
                        println!("Thank you for using this library {username}. See you next time!");
                        break
                    },
                    0 | 5..=u8::MAX => println!("These choices don't exist, please use the numbers provided")
                }
            }
            Err(_err) => println!("was unable to parse as a valid number")
        } 
    }
}
