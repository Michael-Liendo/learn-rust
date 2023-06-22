use rand::Rng;
use std::io;

fn main() {
    let characters =
        "0123456789abcdefghijklmnopqrstuvwxyz!@#$%^&*()ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    let mut password_length = String::new();

    println!("Enter a password characters length:");

    let _ = io::stdin().read_line(&mut password_length);

    let mut password = String::new();

    for _i in 0..password_length.as_str().trim().parse().unwrap() {
        let number = rand::thread_rng().gen_range(0..characters.len());

        let char = characters.chars().nth(number).unwrap();

        password.push(char)
    }
    println!("{}", password);
}
