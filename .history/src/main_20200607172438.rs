use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_num = rand

    println!("Please input your guess.");

    let mut guess = String::new(); // associated function == static function

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
