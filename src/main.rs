use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number: {}", secret_number);
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    loop {
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
