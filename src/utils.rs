use std::io;

pub fn remove_trailing_whitespace(s: &mut String) {
    if s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
        remove_trailing_whitespace(s);
    }
}

pub fn get_input(msg: &str) -> String {
    let mut input = String::new();
    println!("{msg}");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    remove_trailing_whitespace(&mut input);
    input
}