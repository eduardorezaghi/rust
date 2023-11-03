use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        // If an error occurs, the program will crash and display the message passed to expect.
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
