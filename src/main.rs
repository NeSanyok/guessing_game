use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    'outer: loop {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is: {secret_number}");
        'inner: loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue 'inner,
            };

            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    println!("Wanna try again? Y/N");
                    'inner_inner: loop {
                        let mut again: String = String::new();

                        io::stdin()
                            .read_line(&mut again)
                            .expect("Failed to read line");

                        let again = again.trim();
                        if again == "Y" {
                            println!("Restarting...");
                            continue 'outer;
                        } else if again == "N" {
                            println!("Game over");
                            break 'outer;
                        } else {
                            println!("I accept only Y or N values");
                            continue 'inner_inner;
                        }
                    }
                }
            }
        }
    }
}
