use std::io; // bring the io input/output library into scope from the standard librarys std:
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // mut -> to make the variables mutable
        // :: in String::new indicates that new is an associated function of the String type, which creates a new, empty string, a common pattern for functions that initialize new values of a type.

        io::stdin()
            .read_line(&mut guess) // & symbol indicates that the argument is a reference, allowing multiple parts of your code to access the same data without copying it
            .expect("Failed to read line"); // uses result value(enum) from read_line to handle error

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input invalid -> not a number");
                continue; // continue loop after invalid input
            }
        }; // shadow the previous value of guess (from String to Integer)

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }
    }
}
