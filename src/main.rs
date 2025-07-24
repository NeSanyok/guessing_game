use rand::Rng;
use std::io;
fn easy() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    //fn restart(again)

    fn handle_guess(secret_number: u32) {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                return handle_guess(secret_number);
            }
        };

        if guess < secret_number {
            println!("Nigger");
            handle_guess(secret_number);
        } else if guess > secret_number {
            println!("Smaller");
            handle_guess(secret_number);
        } else if guess == secret_number {                  //guessed!
            println!("You won!");
            println!("Wanna play again? Y/N");
            fn restart() {
                let mut again = String::new();
                io::stdin()
                    .read_line(&mut again)
                    .expect("Failed to read line");
                let again = again.trim();
                if again == "Y" {
                    easy()
                } else if again == "N" {
                    println!("Well played");
                } else {
                    println!("Invalid input (Y/N)");
                    restart();
                }
            }
            restart() // Function won't start by itself
        } else {
            println!("Sorry, looks like something went wrong, crashing the program...");
        }
    }
    handle_guess(secret_number); // Function won't start by itself
}
fn medium() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=1000);

    println!("The secret number is: {secret_number}");

    //fn restart(again)

    fn handle_guess(secret_number: u32) {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                return handle_guess(secret_number);
            }
        };

        if guess < secret_number {
            println!("Nigger");
            handle_guess(secret_number);
        } else if guess > secret_number {
            println!("Smaller");
            handle_guess(secret_number);
        } else if guess == secret_number {                  //guessed!
            println!("You won!");
            println!("Wanna play again? Y/N");
            fn restart() {
                let mut again = String::new();
                io::stdin()
                    .read_line(&mut again)
                    .expect("Failed to read line");
                let again = again.trim();
                if again == "Y" {
                    easy()
                } else if again == "N" {
                    println!("Well played");
                } else {
                    println!("Invalid input (Y/N)");
                    restart();
                }
            }
            restart() // Function won't start by itself
        } else {
            println!("Sorry, looks like something went wrong, crashing the program...");
        }
    }
    handle_guess(secret_number); // Function won't start by itself
}
fn hard() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=6969);

    println!("The secret number is: {secret_number}");

    //fn restart(again)

    fn handle_guess(secret_number: u32) {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                return handle_guess(secret_number);
            }
        };

        if guess < secret_number {
            println!("Nigger");
            handle_guess(secret_number);
        } else if guess > secret_number {
            println!("Smaller");
            handle_guess(secret_number);
        } else if guess == secret_number {                  //guessed!
            println!("You won!");
            println!("Wanna play again? Y/N");
            fn restart() {
                let mut again = String::new();
                io::stdin()
                    .read_line(&mut again)
                    .expect("Failed to read line");
                let again = again.trim();
                if again == "Y" {
                    easy()
                } else if again == "N" {
                    println!("Well played");
                } else {
                    println!("Invalid input (Y/N)");
                    restart();
                }
            }
            restart() // Function won't start by itself
        } else {
            println!("Sorry, looks like something went wrong, crashing the program...");
        }
    }
    handle_guess(secret_number); // Function won't start by itself
}
fn main() {
    println!("Select your difficulty: easy/medium/hard");

    let mut difficulty = String::new();

    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");

    let difficulty = difficulty.trim();

    if difficulty == "easy" {
        easy();
    } else if difficulty == "medium" {
        medium();
    } else if difficulty == "hard" {
        hard()
    } else {
        println!("Restart and write easy or medium or hard")
    }
}
