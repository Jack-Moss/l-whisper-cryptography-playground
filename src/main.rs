use std::io;

use crate::encryptor::Encryptable;
pub mod encryptor;
fn main() {
    println!("input the string you wanna encrypt: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("cannot read user input");
    println!("Your encrypted string: {}", encryptor::rot13::Rot13(user_input).encrypt());
}
