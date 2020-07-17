fn reciprocal_char(c: char) -> char {
    match c {
        'a'..='z' => (25 - (c as u8 - 'a' as u8) + 'a' as u8) as char,
        'A'..='Z' => (25 - (c as u8 - 'A' as u8) + 'A' as u8) as char,
        '0'..='9' => (9 - (c as u8 - '0' as u8) + '0' as u8) as char,
        _ => c,
    }
}

fn main() {
    println!("Enter the string: ");
    let input = inputparser::stringinput();
    let reciprocated: String = input.chars().map(reciprocal_char).collect();
    println!("Here's your string: {}", reciprocated);
}
