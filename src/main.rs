use std::io;

fn reciprocal_char(c: char) -> char {
    match c {
        'a'..='z' => (25 - (c as u8 - 'a' as u8) + 'a' as u8) as char,
        'A'..='Z' => (25 - (c as u8 - 'A' as u8) + 'A' as u8) as char,
        '0'..='9' => (9 - (c as u8 - '0' as u8) + '0' as u8) as char,
        _ => c,
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    println!("Enter the string: ");
    stdin.read_line(&mut input).expect("Error reading line!");
    let reciprocated: String = input.chars().map(reciprocal_char).collect();
    println!("Here's your string: {}", reciprocated);
}