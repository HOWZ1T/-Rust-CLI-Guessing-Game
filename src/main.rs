use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    // welcome prompts to the user
    println!("Welcome to: Guess that number!");
    println!("I will now choose a random number from 1 to 100, and it is your job to guess it!");
    println!("Remember you can quit at anytime by typing in: 'exit' or 'quit'");
    println!("Let's begin!");

    // use rand crate to generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // the number of attempts the user had at guessing the number
    let mut attempts: u32 = 0;

    loop {
        print!("Input your guess: ");
        std::io::stdout().flush().expect("failed to flush stdout!");

        // getting the user's input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        guess = guess.to_lowercase();
        let guess: &str = guess.trim();

        // checking for exit keywords
        if guess == "exit" || guess == "quit" {
            println!("goodbye! :)");
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                // skips current loop iteration when an invalid number is entered by the user
                println!("invalid input! please enter a number!");
                continue;
            }
        };

        attempts += 1;
        println!("You guessed: {}", guess);

        // comparing the number against our secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                let feedback: &str;
                // based on number of attempts generate custom feedback string for the user
                match attempts.cmp(&(10 as u32)) {
                    Ordering::Less => feedback = "Well done! ヘ( ^o^)ノ＼(^_^ )",
                    Ordering::Greater => {
                        feedback =
                            "Not great, you were way off the mark! (╯°□°）╯︵ ┻━┻"
                    }
                    Ordering::Equal => feedback = "Not bad, not great either. ¯\\(°_o)/¯",
                }

                println!(
                    "You win! The number was: {}\nYou guessed it in {} tries. {}\nGoodbye! :)",
                    &secret_number, &attempts, feedback
                );
                break; // exit on win condition
            }
        }
    }
}
