use std::io;
use cipher::{rot13,Cipher};
fn main() {
    println!("input the string you wanna encrypt: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("cannot read user input");
    println!("Your encrypted string: {}", 
    cipher::rot13::Rot13(user_input).encrypted_string().unwrap());

    let mut user_input = String::new();

    io.stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input");

    let encrypted_input = rsa::Rsa::new(
        user_input).expect("");
    let encrypted_string = encrypted_input.encrypted_string("");
    print!("Your encrypted string: {}",encrypted_string);
    let decrypted_string = encrypted_input.original_string().expect("");
    println!("your original string: {}", decrypted_string);
}
