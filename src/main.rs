use std::io;
use rand::Rng;
use colored::*;

fn main() {
    println!("{}", "----- Guess the number ! -----".bold());

    let mut guess: u32 = 0;

    //generate the secret number 
    let secret_number = generate_secret_number();

    while guess != secret_number {
        guess = get_user_input();

        println!("You guessed: {}", guess);

        if guess > secret_number {
            println!("{}", "Too big !".red());
        } else if guess < secret_number {
            println!("{}", "Too small !".red());
        }
    }

    println!("{} {}", "Well play ! The secret number was: ".green(), secret_number.to_string().green().bold());
}

fn generate_secret_number() -> u32 {
    let secret_number =  rand::thread_rng().gen_range(1..100);
    return secret_number;
}

fn get_user_input() -> u32 {
    let mut guess = String::new();

    while true {
        println!("Please enter your guess: ");
        io::stdin().read_line(&mut guess).expect("Failed to read line !");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                return num;
            },
            Err(_) => {
                println!("{}",  " Please enter a valid number !".red().bold());
                continue;
            },
        };
    };

    return 0;
}