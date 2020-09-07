use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("WELCOME TO THE GUESSING GAME");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please enter a your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read your guess...");
        let guess: u32 = guess.trim().parse().expect("Please type a guess number");
        println!("You guessed the number: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess of {} was less than the random number", guess),
            Ordering::Greater => {
                println!("Your guess of {} was greater than the random number", guess)
            }
            Ordering::Equal => {
                println!("Your guess of {} was correct! YOU WIN", guess);
                println!("Now exiting the game...");
                break;
            }
        }
    }
}
