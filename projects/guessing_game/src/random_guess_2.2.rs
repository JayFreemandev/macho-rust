use std::io;
use rand::Rng;

fn random() {
    println!("Guess the number!");

    let secret_number = rand::rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guessed: {}", guess);
}
