use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        
        // Create a mutable string
        let mut guess = String::new();
        
        // stdin() returns a handle to the standard input for the terminal
        io::stdin()
        // pass in a reference (&) to guess. Must specify mutable reference (&mut)
        .read_line(&mut guess)
        // read_line() returns an enumaration (enum) which is a type that can be in one
        // of multiple possible states (variants). Result enum has variants OK and Err.
        // Result type has method expect(str) which will crash the program and print str
        // if the Result is an Err. Otherwise it will take the return value that OK holds
        // and return that for use (in this case the number of bytes in the user input)
        .expect("Failed to read line");
        
        // shadowing the guess variable to convert types
        // .trim() eliminates white space (in this case the \n)
        // .parse() attempts to parse the sring to the type specified by the variable it is going to be bound to
        // we run a match on the enum returned by .parse() to handle the results
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ in the Err(_) is a catchall value - lets us match any Err value
        };
        
        println!("You guessed: {guess}");
        
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