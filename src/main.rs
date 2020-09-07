use std::io;

fn main() {
    println!("WELCOME TO THE GUESSING GAME");

    println!("Please enter a number.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read your guess...");
    println!("You guessed the number: {}", guess);
}
