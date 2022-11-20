use std::io;
use super::utils::get_input;

pub fn print_books (collection: &Vec<[String; 9]>) {
    println!("\nYou currently have {} books: ", collection.len());
    for (idx, book) in collection.iter().enumerate() {
        let mut output = format!(
            "{}. {} by {}: {}", idx+1, book[0], book[1], book[2]
        );
        if book[2] == "Rented" {
            let rent_deet = format!(" | {} due on {}", book[8], book[7]);
            output.push_str(&rent_deet);
        }
        if book[2] == "Borrowed" {
            let borrow_deet = format!(" | return by {}", book[5]);
            output.push_str(&borrow_deet);
        }
        println!("{}", output);
    }
}

fn create_book(author: String, 
    title: String, 
    book_type: String,
    owned_date: Option<String>,
    s_date: Option<String>,
    e_date: Option<String>,
    r_price: Option<String>
) -> [String; 9] {
    if book_type == "Owned" {
        [
            title.to_string(), 
            author.to_string(), 
            book_type.to_string(),
            owned_date.unwrap_or("Unset".to_string()).to_string(),
            String::new(), // borrowed start date
            String::new(), // borrowed end date
            String::new(), // rent start date
            String::new(), // rent end date
            String::new(), // rent price
        ]
    } else if book_type == "Rented" {
        [
            title.to_string(), 
            author.to_string(), 
            book_type.to_string(),
            String::new(),
            String::new(), // borrowed start date
            String::new(), // borrowed end date
            s_date.unwrap_or("Unset".to_string()).to_string(), // rent start date
            e_date.unwrap_or("Unset".to_string()).to_string(), // rent end date
            r_price.unwrap_or("Unset".to_string()).to_string(), // rent price
        ]
    } else if book_type == "Borrowed" {
        [
            title.to_string(),
            author.to_string(),
            book_type.to_string(),
            String::new(),
            s_date.unwrap_or("Unset".to_string()).to_string(),
            e_date.unwrap_or("Unset".to_string()).to_string(),
            String::new(),
            String::new(),
            String::new(),
        ]
    } else {
        [
            String::new(),
            String::new(),
            "Failed".to_string(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ]
    }
}

pub fn add_book (collection: &mut Vec<[String; 9]>, n_book: &mut u8) { 
    println!("What are you adding?\n\
    \t1. Owned book
    \t2. Rented book
    \t3. Borrowed book \
    ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input = input.trim().to_string();
    let choice: u8 = match input.parse::<u8>() {
        Ok(ok) => ok,
        Err(_err) => 0
    };

    if choice == 0 {
        println!("Input was not part of choices")
    } 
    let author = get_input("Enter Book's Author");
    let title = get_input("Enter Book's Title");

    if choice == 1 {
        let owned_date = get_input("Enter Date");
        let book = create_book(author, 
            title, 
            "Owned".to_string(), 
            Some(owned_date), 
            None, 
            None,
            None,
        );
        collection.push(book);
    } else if choice == 2 {
        let s_date = get_input("Enter Date when rented");
        let e_date = get_input("Enter Date when book's due");
        let r_price = get_input("Enter renting price");
        let book = create_book(author, 
            title, 
            "Rented".to_string(), 
            None, 
            Some(s_date), 
            Some(e_date),
            Some(r_price),
        );
        collection.push(book);
    } else if choice == 3 {
        let s_date = get_input("Enter Date when rented");
        let e_date = get_input("Enter Date when book's due");
        let book = create_book(author, 
            title, 
            "Borrowed".to_string(), 
            None, 
            Some(s_date), 
            Some(e_date),
            None,
        );
        collection.push(book);
    }
    *n_book += 1;

}

pub fn remove_book(collection: &mut Vec<[String; 9]>, n_book: &mut u8) {
    print_books(collection);
    let input = get_input("Enter the number of the book to be removed:");
    let choice: u8 = match input.parse::<u8>() {
        Ok(ok) => ok,
        Err(_err) => u8::MAX
    };
    if choice > *n_book || choice == 0 {return println!("Number not part of choices")}
    
    collection.remove((choice - 1).into());
}