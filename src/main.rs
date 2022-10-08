// prelude
// io input/output library that comes from the standard library std
use std::io;
use rand::Rng; // The Rng trait defines methods
use std::cmp::Ordering; // enum (Less, Greater, Equal)

fn main() {
    println!("Guess the number!");

    // i32 by default, but Rust will infer that number should be a u32
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // result of calling String::new, a function that returns a new instance of a String
        // new is an associated function of the String type (implemented on a String type)
        let mut guess = String::new();

        // stdin function from io module, returns an instance of std::io::Stdin - 
        // a type that represents a handle to the standard input for your terminal
        io::stdin() // or std::io::stdin without use
            // store string, read_line method can change the content, returns a Result value (an enumeration
            .read_line(&mut guess) // - a type that can be in one of multiple possible states (variants Ok and Err))
            .expect("Failed to read line");

        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ - catchall value; go to the next iteration of the loop
        };
        // trim eliminates any whitespace at the beginning and end
        // String.parse converts to a number: u32; returns a Result

        println!("You guessed: {guess}");

        // use match expression - what to do next?
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // an arm consists of a pattern
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
