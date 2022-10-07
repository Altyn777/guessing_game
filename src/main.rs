// prelude
// io input/output library that comes from the standard library std
use std::io;

fn main() {
    println!("Guess the number!");

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

    println!("You guessed: {guess}");
}
