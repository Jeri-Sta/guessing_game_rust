use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1.. 101);
    println!("Guess the number!");

    loop {
        println!("Type your number to guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read palpite");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
}
